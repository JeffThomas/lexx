use criterion::{Criterion, black_box, criterion_group, criterion_main};
use lexxor::rolling_char_buffer::RollingCharBuffer;

fn bench_push_pop(c: &mut Criterion) {
    c.bench_function("push_pop", |b| {
        b.iter(|| {
            let mut buf = RollingCharBuffer::<1024>::new();
            for i in 0..1024 {
                buf.push(black_box((i % 256) as u8 as char)).unwrap();
            }
            for _ in 0..1024 {
                black_box(buf.pop().unwrap());
            }
        })
    });
}

fn bench_prefix_read(c: &mut Criterion) {
    c.bench_function("prefix_read", |b| {
        b.iter(|| {
            let mut buf = RollingCharBuffer::<1024>::new();
            for i in 0..1024 {
                buf.prefix(black_box((i % 256) as u8 as char)).unwrap();
            }
            for _ in 0..1024 {
                black_box(buf.read().unwrap());
            }
        })
    });
}

fn bench_extend_clear(c: &mut Criterion) {
    c.bench_function("extend_clear", |b| {
        b.iter(|| {
            let mut buf = RollingCharBuffer::<1024>::new();
            let data: Vec<char> = (0..1024).map(|i| (i % 256) as u8 as char).collect();
            buf.extend(&data).unwrap();
            buf.clear();
        })
    });
}

fn bench_prepend_pop(c: &mut Criterion) {
    c.bench_function("prepend_pop", |b| {
        b.iter(|| {
            let mut buf = RollingCharBuffer::<1024>::new();
            let data: Vec<char> = (0..1024).map(|i| (i % 256) as u8 as char).collect();
            buf.prepend(&data).unwrap();
            for _ in 0..1024 {
                black_box(buf.pop().unwrap());
            }
        })
    });
}

fn bench_mixed_ops(c: &mut Criterion) {
    c.bench_function("mixed_ops", |b| {
        b.iter(|| {
            let mut buf = RollingCharBuffer::<1024>::new();
            for i in 0..512 {
                buf.push(black_box((i % 256) as u8 as char)).unwrap();
            }
            for i in 0..256 {
                buf.prefix(black_box((i % 256) as u8 as char)).unwrap();
            }
            for _ in 0..128 {
                black_box(buf.read().unwrap());
            }
            for _ in 0..128 {
                black_box(buf.pop().unwrap());
            }
            let data: Vec<char> = (0..256).map(|i| (i % 256) as u8 as char).collect();
            buf.extend(&data).unwrap();
            buf.prepend(&data).unwrap();
        })
    });
}

fn bench_full_buffer_handling(c: &mut Criterion) {
    c.bench_function("full_buffer_handling", |b| {
        b.iter(|| {
            let mut buf = RollingCharBuffer::<128>::new();
            for i in 0..128 {
                buf.push(black_box((i % 64) as u8 as char)).unwrap();
            }
            // Now buffer is full, next push should fail
            assert!(buf.push('x').is_err());
            for _ in 0..128 {
                black_box(buf.read().unwrap());
            }
            assert!(buf.read().is_err());
        })
    });
}

criterion_group!(
    benches,
    bench_push_pop,
    bench_prefix_read,
    bench_extend_clear,
    bench_prepend_pop,
    bench_mixed_ops,
    bench_full_buffer_handling,
);
criterion_main!(benches);
