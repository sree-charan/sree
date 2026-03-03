// Theme system tests

use sree::ui::theme::Theme;

#[test]
fn test_dark_theme() {
    let theme = Theme::dark();
    assert_eq!(theme.name, "dark");
}

#[test]
fn test_light_theme() {
    let theme = Theme::light();
    assert_eq!(theme.name, "light");
}

#[test]
fn test_monokai_theme() {
    let theme = Theme::monokai();
    assert_eq!(theme.name, "monokai");
}

#[test]
fn test_dracula_theme() {
    let theme = Theme::dracula();
    assert_eq!(theme.name, "dracula");
}

#[test]
fn test_nord_theme() {
    let theme = Theme::nord();
    assert_eq!(theme.name, "nord");
}

#[test]
fn test_solarized_theme() {
    let theme = Theme::solarized();
    assert_eq!(theme.name, "solarized");
}

#[test]
fn test_theme_from_name() {
    let dark = Theme::from_name("dark");
    assert_eq!(dark.name, "dark");
    
    let light = Theme::from_name("light");
    assert_eq!(light.name, "light");
    
    let monokai = Theme::from_name("monokai");
    assert_eq!(monokai.name, "monokai");
    
    let dracula = Theme::from_name("dracula");
    assert_eq!(dracula.name, "dracula");
    
    let nord = Theme::from_name("nord");
    assert_eq!(nord.name, "nord");
    
    let solarized = Theme::from_name("solarized");
    assert_eq!(solarized.name, "solarized");
    
    // Unknown theme defaults to dark
    let unknown = Theme::from_name("unknown");
    assert_eq!(unknown.name, "dark");
}

#[test]
fn test_available_themes() {
    let themes = Theme::available_themes();
    assert_eq!(themes.len(), 6);
    assert!(themes.contains(&"dark"));
    assert!(themes.contains(&"light"));
    assert!(themes.contains(&"monokai"));
    assert!(themes.contains(&"dracula"));
    assert!(themes.contains(&"nord"));
    assert!(themes.contains(&"solarized"));
}
