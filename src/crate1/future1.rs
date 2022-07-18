use std::{sync::{Mutex, Arc, mpsc::{SyncSender, sync_channel, Receiver}}, task::{Waker, Context, Poll}, pin::Pin, time::Duration, thread};

use futures::{Future, future::BoxFuture, task::{ArcWake, waker_ref}, FutureExt};


//构建执行器，运行逻辑：
//1，将async代码块包裹成Task发送到管道中
//2，executor从通道拿出Task，将Task转换成Waker再转换成Context，传入Task.future的poll()中并调用
//3，因为TimerFuture::new后面接await，先异步等待运行TimerFuture::new结束，其中也会在子线程中休眠然后调用wake()。
// 于是这个时候先返回TimerFuture实例，输出TimerFuture::new done!，先调用其poll()，此时completed是false，
// 因此输出TimeFuture poll bad!返回Poll::Pending并且把future又赋给了Task。
//4，如果这个时候注释掉Task的wake()实现，那就结束了，因为TimerFuture::new的子线程里调用wake()是空实现。
//5，如果不注释Task的wake()，则在TimerFuture::new的子线程里将completed设置为true后调用wake()，
// 将Task再发送进管道，这样Executor的run()又会收到消息。再走一遍流程，这个时候poll结果就是Poll::Ready，输出TimeFuture poll ok!。
//输出：
// howdy!
// TimerFuture::new done!
// TimeFuture poll bad!
// TimeFuture poll ok!
// done!
//注释掉Task的wake()实现，则输出：
// howdy!
// TimerFuture::new done!
// TimeFuture poll bad!
#[allow(dead_code)]
pub fn f1(){
    let (executor, spawner) = new_executor_and_spawner();
    spawner.spawn(async{
        println!("howdy!");
        TimerFuture::new(Duration::new(2,0)).await;
        println!("done!");
    });
    drop(spawner); //spawner的spawn已经将future发送到通道里面去了，所以spawner没用了。
    executor.run();
}

struct TimerFuture{
    shared_state:Arc<Mutex<SharedState>>,
}

struct SharedState{
    completed:bool,
    waker:Option<Waker>,
}

impl Future for TimerFuture{
    type Output=();
    fn poll(self:Pin<&mut Self>,cx:&mut Context<'_>)->Poll<Self::Output>{
        //上锁
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed{
            println!("TimeFuture poll ok!");
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone()); //Waker从Context里面来
            println!("TimeFuture poll bad!");
            Poll::Pending
        }
    }
}

impl TimerFuture{
    //休眠duration，标记，调用shared_state.waker导致调用TimeFuture的poll
    fn new(duration:Duration)->Self{
        let shared_state=Arc::new(Mutex::new(SharedState{
            completed:false,
            waker:None,
        }));
        let thread_shared_state=shared_state.clone();
        thread::spawn(move||{
            thread::sleep(duration);//实现定时器
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed=true;
            if let Some(waker)=shared_state.waker.take(){
                waker.wake() //调用wake()
            }
        });
        println!("TimerFuture::new done!");
        TimerFuture{shared_state}
    }
}

struct Executor{
    ready_queue:Receiver<Arc<Task>>,
}

#[derive(Clone)]
struct Spawner{
    task_sender:SyncSender<Arc<Task>>,
}

impl Spawner{
    //接收一个future引用，装箱后和sender一起构建Task然后用sender发送进管道
    fn spawn(&self,future:impl Future<Output=()>+'static+Send){
        let future = future.boxed();
        let task = Arc::new(Task{ //构建Task
            future:Mutex::new(Some(future)),
            task_sender:self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

#[allow(dead_code)]
struct Task{
    future:Mutex<Option<BoxFuture<'static,()>>>,
    task_sender:SyncSender<Arc<Task>>,
}

#[allow(dead_code)]
impl ArcWake for Task{
    fn wake_by_ref(arc_self:&Arc<Self>){
        let cloned=arc_self.clone();
        //这里发送的Task，Task中含有通道的sender，这样就实现了通信
        arc_self.task_sender.send(cloned).expect("too many tasks queued");
    }
}

fn new_executor_and_spawner()->(Executor,Spawner) {
    const MAX_QUEUED_TASKS:usize=10_000;
    let (task_sender,ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor{ready_queue},Spawner{task_sender})
}

impl Executor{
    fn run(&self){
        //循环从管道中拿消息
        while let Ok(task) = self.ready_queue.recv(){
            //加锁
            let mut future_slot=task.future.lock().unwrap();
            //这里take的是Some表示这个future没有完成，比如在代码最下方被赋值了
            if let Some(mut future)=future_slot.take(){
                //这里的waker是Arc<impl ArcWake>的一个引用，也就是Arc<Task>的一个引用
                //创建的Waker中的RawWaker中的data即是该&task，vtable则是上面的impl ArcWake for Task
                let waker = waker_ref(&task);
                //创建一个Context实例的可变引用
                let context = &mut Context::from_waker(&*waker);
                //调用future的poll，如果返回的还是Poll::Pending就将Some(future)的引用再赋给future_slot，
                //这个时候future_slot是空的，留待下次再运行
                if let Poll::Pending=future.as_mut().poll(context){
                    *future_slot = Some(future);
                }
            }
        }
    }
}