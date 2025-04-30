use itertools::Itertools;

pub const PADDING: usize = 4;

pub fn dedup_packages(packages: Vec<String>) -> Vec<String> {
    packages.into_iter().dedup().collect::<Vec<String>>()
}

#[macro_export]
macro_rules! print_res {
    ( unannotated $pkgsite:expr, $method:ident, $struct:ty, $packages:expr $(, $arg:expr)* ) => {
        let dedup = dedup_packages($packages);
        let inner = $pkgsite.$method(&dedup $(, $arg)*).await?;
        println!(
            "{}",
            inner
                .iter()
                .map(|i| <$struct>::from(i).to_string())
                .collect::<Vec<String>>()
                .join("\n\n")
        );
    };

    ( annotated $pkgsite:expr, $method:ident, $struct:ty, $packages:expr $(, $arg:expr)* ) => {
        let dedup = dedup_packages($packages);
        let inner = $pkgsite.$method(&dedup $(, $arg)*).await?;
        println!(
            "{}",
            inner
                .iter()
                .map(|(pkg, i)| format!("{}:\n{}", pkg, <$struct>::from(i)))
                .collect::<Vec<String>>()
                .join("\n\n")
        );
    };

    ( single $pkgsite:expr, $method:ident, $struct:ty $(, $arg:expr)* ) => {
        let inner = $pkgsite.$method($($arg, )*).await?;
        println!("{}", <$struct>::from(&inner).to_string());
    };
}
