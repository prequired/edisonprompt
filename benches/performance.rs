use criterion::{black_box, criterion_group, criterion_main, Criterion};
use promptedo::{Database, TemplateEngine};
use tempfile::TempDir;
use std::collections::HashMap;

fn benchmark_startup(c: &mut Criterion) {
    c.bench_function("startup_time", |b| {
        b.iter(|| {
            // Simulate CLI startup
            let _temp_dir = TempDir::new().unwrap();
            // This should complete in <10ms
        });
    });
}

fn benchmark_search(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let mut database = Database::new(temp_dir.path().join("test.db")).unwrap();
    
    // Add 1000 test prompts
    for i in 0..1000 {
        let prompt = promptedo::database::models::Prompt::new(
            format!("test-prompt-{}", i),
            format!("This is test content number {} for searching", i),
        );
        database.create_prompt(&prompt).unwrap();
    }
    
    c.bench_function("search_1000_prompts", |b| {
        b.iter(|| {
            let results = database.search_prompts(black_box("test"), 50, false).unwrap();
            black_box(results);
        });
    });
}

fn benchmark_template_rendering(c: &mut Criterion) {
    let engine = TemplateEngine::new();
    let template = "Hello {{name}}, your {{type}} order for {{item}} is {{status}}!";
    let mut variables = HashMap::new();
    variables.insert("name".to_string(), "John Doe".to_string());
    variables.insert("type".to_string(), "priority".to_string());
    variables.insert("item".to_string(), "laptop".to_string());
    variables.insert("status".to_string(), "ready".to_string());
    
    c.bench_function("template_render", |b| {
        b.iter(|| {
            let result = engine.render(black_box(template), black_box(&variables)).unwrap();
            black_box(result);
        });
    });
}

fn benchmark_variable_extraction(c: &mut Criterion) {
    let engine = TemplateEngine::new();
    let template = "Complex template with {{var1}} and {{var2}} and {{var3}} variables for {{purpose}}";
    
    c.bench_function("variable_extraction", |b| {
        b.iter(|| {
            let variables = engine.extract_variables(black_box(template)).unwrap();
            black_box(variables);
        });
    });
}

criterion_group!(
    benches,
    benchmark_startup,
    benchmark_search,
    benchmark_template_rendering,
    benchmark_variable_extraction
);
criterion_main!(benches);