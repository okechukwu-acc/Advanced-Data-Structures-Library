use criterion::{Criterion, black_box};

pub fn benchmark_function<F>(c: &mut Criterion, name: &str, f: F)
where
    F: Fn() + 'static,
{
    c.bench_function(name, |b| b.iter(|| f()));
}

pub fn benchmark_data_structure_operations<T, F>(c: &mut Criterion, name: &str, setup: F, operations: Vec<(&str, T)>)
where
    F: Fn() -> T + 'static,
    T: 'static,
{
    for (op_name, operation) in operations {
        c.bench_with_input(name, &op_name, |b, _| {
            let mut data_structure = setup();
            b.iter(|| operation(&mut data_structure))
        });
    }
}
