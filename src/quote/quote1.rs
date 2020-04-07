use quote::quote;

//三方库quote的测试，简单来书就是替换#开头的变量输出TokenStream
pub fn q_test1(){
    let generics = "";
    let where_clause = "";
    let field_ty = "";
    let item_ty = "";
    let path = "";
    let value = "";
    let tokens = quote! {
        struct SerializeWith #generics #where_clause {
            value: &'a #field_ty,
            phantom: core::marker::PhantomData<#item_ty>,
        }
        impl #generics serde::Serialize for SerializeWith #generics #where_clause {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                #path(self.value, serializer)
            }
        }
        SerializeWith {
            value: #value,
            phantom: core::marker::PhantomData::<#item_ty>,
        }
    };
    println!("{}",tokens.to_string());
    //struct SerializeWith "" "" {value : & 'a "" , phantom : core :: marker :: PhantomData < "" > , }
    // impl "" serde :: Serialize for SerializeWith "" "" {
    // fn serialize < S > ( & self , serializer : S ) -> Result < S :: Ok , S :: Error >
    // where S : serde :: Serializer , { "" ( self . value , serializer ) } }
    // SerializeWith { value : "" , phantom : core :: marker :: PhantomData :: < "" > , }
}