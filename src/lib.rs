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
            $lines:stmt;
        )*
    ) => {
            let start = Instant::now();
            $($lines;)*
            let duration = start.elapsed();
            println!("{} time: {:?}", stringify!($($name)?), duration);
        }
}

#[macro_export]
macro_rules! runcount {
    (
        #[$($name:ident)?,$timesec:expr,$timenano:expr]
        $($lines:stmt;)*
    ) => {
        let dur = Duration::new($timesec, $timenano);
        let start = Instant::now();
        let mut count = 0;
        loop {
            count += 1;
        $($lines;)*
        if start.elapsed() >= dur {
                let totaldur = start.elapsed();
                println!("ran {} for {} times in {:?} time.", stringify!($($name)?), count, totaldur);
                break;
            }
        }
    };
}

#[cfg(test)]
mod tests {

    use std::time::{Instant, Duration};

    #[test]
    fn test_benchmark() {
        //assert!(stringify!(benchmark!{}) contains "let start = Instant::now();");
        //TODO CHECK FOR VALID OUTPUT
    }
    #[test]
    fn test_benchmark_named() {
        //assert!(stringify!(benchmark!{}) contains "let start = Instant::now();");
        //TODO CHECK FOR VALID OUTPUT
        benchmarknamed! {
            #[IhaveAname]
            for x in 0..10 {
                let y = x * 2;
            };
        }
    }

    #[test]
    fn test_runcount() {
        let mut x = 1;
        runcount! {
            #[testcount, 1, 0]
            let x = x * 2;
        }
        assert_eq!("", "ran testcount for 14222626 times in 1.000000281s time.");
        
        assert!(match "" {
            "ran testcount for 14222626 times in 1.000000281s time." => true,
            _ => false,
        });
    }
}
