#[macro_export]
macro_rules! benchmark {
    ($($lines:stmt;)*) => {
        let start = Instant::now();
        $($lines;)*
        let duration = start.elapsed();
        println!("Benchmark time: {:?}", duration);
    };
}

#[macro_export]
macro_rules! benchmarknamed {
    (
        #[$($name:ident)?]
        $(
            $(#[$inner:ident $($args:tt)*])*
            $lines:stmt;
        )*
    ) => {
            let start = Instant::now();
            $($lines;)*
            let duration = start.elapsed();
            println!("{} time: {:?} | {}", stringify!($($name)?), duration, stringify!(args));
        };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
