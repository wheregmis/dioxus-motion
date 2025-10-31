use criterion::{Criterion, criterion_group, criterion_main};
use dioxus_motion::prelude::*;
use easer::functions::Easing;
use std::hint::black_box;
use std::time::Duration;

fn bench_animation_config_creation(c: &mut Criterion) {
    c.bench_function("animation_config_spring", |b| {
        b.iter(|| {
            let config = AnimationConfig::spring();
            black_box(config);
        });
    });

    c.bench_function("animation_config_tween", |b| {
        b.iter(|| {
            let config = AnimationConfig::tween();
            black_box(config);
        });
    });

    c.bench_function("animation_config_custom_spring", |b| {
        b.iter(|| {
            let config = AnimationConfig::custom_spring(180.0, 25.0, 1.0);
            black_box(config);
        });
    });

    c.bench_function("animation_config_custom_tween", |b| {
        b.iter(|| {
            let config = AnimationConfig::custom_tween(
                Duration::from_millis(300),
                easer::functions::Cubic::ease_in_out,
            );
            black_box(config);
        });
    });
}

fn bench_transform_operations(c: &mut Criterion) {
    c.bench_function("transform_new", |b| {
        b.iter(|| {
            let transform = Transform::new(
                black_box(100.0),
                black_box(50.0),
                black_box(1.5),
                black_box(std::f32::consts::PI / 4.0),
            );
            black_box(transform);
        });
    });

    c.bench_function("transform_identity", |b| {
        b.iter(|| {
            let transform = Transform::identity();
            black_box(transform);
        });
    });

    let t1 = Transform::new(10.0, 20.0, 1.5, 0.5);
    let t2 = Transform::new(30.0, 40.0, 2.0, 1.0);

    c.bench_function("transform_add", |b| {
        b.iter(|| {
            let result = black_box(t1) + black_box(t2);
            black_box(result);
        });
    });

    c.bench_function("transform_sub", |b| {
        b.iter(|| {
            let result = black_box(t1) - black_box(t2);
            black_box(result);
        });
    });

    c.bench_function("transform_mul", |b| {
        b.iter(|| {
            let result = black_box(t1) * black_box(0.5);
            black_box(result);
        });
    });
}

fn bench_color_operations(c: &mut Criterion) {
    c.bench_function("color_new", |b| {
        b.iter(|| {
            let color = Color::new(
                black_box(1.0),
                black_box(0.5),
                black_box(0.25),
                black_box(1.0),
            );
            black_box(color);
        });
    });

    c.bench_function("color_from_rgba", |b| {
        b.iter(|| {
            let color = Color::from_rgba(
                black_box(255),
                black_box(128),
                black_box(64),
                black_box(200),
            );
            black_box(color);
        });
    });

    let c1 = Color::from_rgba(255, 0, 0, 255);
    let c2 = Color::from_rgba(0, 255, 0, 255);

    c.bench_function("color_add", |b| {
        b.iter(|| {
            let result = black_box(c1) + black_box(c2);
            black_box(result);
        });
    });

    c.bench_function("color_sub", |b| {
        b.iter(|| {
            let result = black_box(c1) - black_box(c2);
            black_box(result);
        });
    });

    c.bench_function("color_mul", |b| {
        b.iter(|| {
            let result = black_box(c1) * black_box(0.5);
            black_box(result);
        });
    });
}

fn bench_spring_tween_construction(c: &mut Criterion) {
    c.bench_function("spring_new", |b| {
        b.iter(|| {
            let spring = Spring {
                stiffness: black_box(180.0),
                damping: black_box(25.0),
                mass: black_box(1.0),
                velocity: black_box(0.0),
            };
            black_box(spring);
        });
    });

    c.bench_function("tween_new", |b| {
        b.iter(|| {
            let tween = Tween {
                duration: black_box(Duration::from_millis(300)),
                easing: easer::functions::Cubic::ease_in_out,
            };
            black_box(tween);
        });
    });
}

criterion_group!(
    benches,
    bench_animation_config_creation,
    bench_transform_operations,
    bench_color_operations,
    bench_spring_tween_construction,
);
criterion_main!(benches);
