fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use anyhow::Result;
    use specta_typescript::Typescript;

    #[derive(specta::Type)]
    pub enum MyEither<L, R> {
        Left(L),
        Right(R),
    }

    #[test]
    fn test_simple_export() -> Result<()> {
        let types_path = std::path::Path::new("priv/test/types.ts");
        Typescript::default().export_to(&types_path, &specta::export())?;
        Ok(())
    }

    #[derive(specta::Type)]
    pub struct TypeOne {
        pub a: String,
        pub b: GenericType<i32>,
        #[serde(rename = "cccccc")]
        pub c: MyEnum,
    }

    #[derive(specta::Type)]
    pub struct GenericType<A> {
        pub my_field: String,
        pub generic: A,
    }

    #[derive(specta::Type)]
    pub enum MyEnum {
        A,
        B,
        C,
    }

    #[test]
    fn readme_test() -> Result<()> {
        assert_eq!(
            specta_typescript::export::<TypeOne>(&Typescript::default())?,
            "export type TypeOne = { a: string; b: GenericType<number>; cccccc: MyEnum }".to_string()
        );
        Ok(())
    }
}
