//! Tests for the Templates module

use frentui::templates::TemplateRegistry;
use frentui::ui::dialog::TemplateField;

#[test]
fn test_template_registry_new() {
    let registry = TemplateRegistry::new();
    let templates = registry.list_for_field(TemplateField::RenamingRule);
    assert!(!templates.is_empty(), "Template registry should have templates");
}

#[test]
fn test_template_registry_default() {
    let registry = TemplateRegistry::default();
    let templates = registry.list_for_field(TemplateField::RenamingRule);
    assert!(!templates.is_empty());
}

#[test]
fn test_template_registry_has_existing() {
    let registry = TemplateRegistry::new();
    let templates: std::collections::HashMap<_, _> = registry.list_for_field(TemplateField::RenamingRule).into_iter().collect();
    
    // Test that existing templates are present
    assert!(templates.contains_key(&"lowercase".to_string()));
    assert!(templates.contains_key(&"uppercase".to_string()));
    assert!(templates.contains_key(&"title-case".to_string()));
    assert!(templates.contains_key(&"photo-date".to_string()));
    assert!(templates.contains_key(&"counter-2".to_string()));
}

#[test]
fn test_template_registry_does_not_have_nonexistent() {
    let registry = TemplateRegistry::new();
    let templates: std::collections::HashMap<_, _> = registry.list_for_field(TemplateField::RenamingRule).into_iter().collect();
    assert!(!templates.contains_key(&"nonexistent-template".to_string()));
}

#[test]
fn test_template_registry_list() {
    let registry = TemplateRegistry::new();
    let templates = registry.list_for_field(TemplateField::RenamingRule);
    
    // Should have multiple templates
    assert!(templates.len() > 10);
    
    // Should be sorted by key
    for i in 1..templates.len() {
        assert!(templates[i].0 >= templates[i-1].0, 
                "Templates should be sorted");
    }
}

#[test]
fn test_template_registry_photo_templates() {
    let registry = TemplateRegistry::new();
    let templates: std::collections::HashMap<_, _> = registry.list_for_field(TemplateField::RenamingRule).into_iter().collect();
    
    assert!(templates.contains_key(&"photo-date".to_string()));
    assert!(templates.contains_key(&"photo-counter".to_string()));
    assert!(templates.contains_key(&"photo-datetime".to_string()));
}

#[test]
fn test_template_registry_case_templates() {
    let registry = TemplateRegistry::new();
    let templates: std::collections::HashMap<_, _> = registry.list_for_field(TemplateField::RenamingRule).into_iter().collect();
    
    let lowercase = templates.get(&"lowercase".to_string()).unwrap();
    assert!(lowercase.contains("%L"));
    
    let uppercase = templates.get(&"uppercase".to_string()).unwrap();
    assert!(uppercase.contains("%U"));
    
    let title_case = templates.get(&"title-case".to_string()).unwrap();
    assert!(title_case.contains("%T"));
}

#[test]
fn test_template_registry_counter_templates() {
    let registry = TemplateRegistry::new();
    let templates: std::collections::HashMap<_, _> = registry.list_for_field(TemplateField::RenamingRule).into_iter().collect();
    
    assert!(templates.contains_key(&"counter-2".to_string()));
    assert!(templates.contains_key(&"counter-3".to_string()));
    assert!(templates.contains_key(&"counter-4".to_string()));
    assert!(templates.contains_key(&"counter-prefix".to_string()));
    assert!(templates.contains_key(&"counter-suffix".to_string()));
}

#[test]
fn test_template_registry_date_templates() {
    let registry = TemplateRegistry::new();
    let templates: std::collections::HashMap<_, _> = registry.list_for_field(TemplateField::RenamingRule).into_iter().collect();
    
    assert!(templates.contains_key(&"date-suffix".to_string()));
    assert!(templates.contains_key(&"date-prefix".to_string()));
    assert!(templates.contains_key(&"datetime-suffix".to_string()));
}

#[test]
fn test_template_registry_cleanup_templates() {
    let registry = TemplateRegistry::new();
    let templates: std::collections::HashMap<_, _> = registry.list_for_field(TemplateField::RenamingRule).into_iter().collect();
    
    assert!(templates.contains_key(&"trim-spaces".to_string()));
    assert!(templates.contains_key(&"underscore-to-dash".to_string()));
    assert!(templates.contains_key(&"dash-to-underscore".to_string()));
}
