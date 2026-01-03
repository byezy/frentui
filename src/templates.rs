//! Template registry for rename patterns and match patterns

use std::collections::HashMap;
use crate::ui::dialog::TemplateField;

pub struct TemplateRegistry {
    rename_templates: HashMap<String, String>,
    match_templates: HashMap<String, String>,
    exclusion_templates: HashMap<String, String>,
}

impl TemplateRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut rename_templates = HashMap::new();
        
        // Photo/Image templates
        rename_templates.insert("photo-date".to_string(), "%N_%D.%E".to_string());
        rename_templates.insert("photo-counter".to_string(), "photo_%C3.%E".to_string());
        rename_templates.insert("photo-datetime".to_string(), "%N_%FD_%FH.%E".to_string());
        
        // Document templates
        rename_templates.insert("doc-date".to_string(), "%N_%D.%E".to_string());
        rename_templates.insert("doc-counter".to_string(), "document_%C2.%E".to_string());
        
        // Lowercase templates
        rename_templates.insert("lowercase".to_string(), "%L%N.%E".to_string());
        rename_templates.insert("lowercase-name".to_string(), "%N%L.%E".to_string());
        
        // Uppercase templates
        rename_templates.insert("uppercase".to_string(), "%U%N.%E".to_string());
        rename_templates.insert("uppercase-name".to_string(), "%N%U.%E".to_string());
        
        // Title case templates
        rename_templates.insert("title-case".to_string(), "%T%N.%E".to_string());
        rename_templates.insert("title-case-name".to_string(), "%N%T.%E".to_string());
        
        // Parent directory templates
        rename_templates.insert("parent-prefix".to_string(), "%P_%N.%E".to_string());
        rename_templates.insert("parent-suffix".to_string(), "%N_%P.%E".to_string());
        
        // Counter templates
        rename_templates.insert("counter-2".to_string(), "%C2.%E".to_string());
        rename_templates.insert("counter-3".to_string(), "%C3.%E".to_string());
        rename_templates.insert("counter-4".to_string(), "%C4.%E".to_string());
        rename_templates.insert("counter-prefix".to_string(), "%C3_%N.%E".to_string());
        rename_templates.insert("counter-suffix".to_string(), "%N_%C3.%E".to_string());
        
        // Date/time templates
        rename_templates.insert("date-suffix".to_string(), "%N_%D.%E".to_string());
        rename_templates.insert("date-prefix".to_string(), "%D_%N.%E".to_string());
        rename_templates.insert("datetime-suffix".to_string(), "%N_%D_%H.%E".to_string());
        
        // Cleanup templates
        rename_templates.insert("trim-spaces".to_string(), "%M%N.%E".to_string());
        rename_templates.insert("underscore-to-dash".to_string(), "%N%R/_/-.%E".to_string());
        rename_templates.insert("dash-to-underscore".to_string(), "%N%R/-/_.%E".to_string());
        
        // Match pattern templates (file selection patterns)
        let mut match_templates = HashMap::new();
        // Non-recursive patterns (current directory only)
        match_templates.insert("all-files".to_string(), "*.*".to_string());
        match_templates.insert("text-files".to_string(), "*.txt".to_string());
        match_templates.insert("images".to_string(), "*.{jpg,jpeg,png,gif,bmp,webp}".to_string());
        match_templates.insert("photos".to_string(), "*.{jpg,jpeg,png}".to_string());
        match_templates.insert("documents".to_string(), "*.{pdf,doc,docx,odt}".to_string());
        match_templates.insert("archives".to_string(), "*.{zip,tar,gz,bz2,7z,rar}".to_string());
        match_templates.insert("videos".to_string(), "*.{mp4,avi,mkv,mov,wmv,flv}".to_string());
        match_templates.insert("audio".to_string(), "*.{mp3,wav,flac,ogg,m4a}".to_string());
        match_templates.insert("code".to_string(), "*.{rs,py,js,ts,java,cpp,c,go}".to_string());
        match_templates.insert("scripts".to_string(), "*.{sh,bash,py,pl,rb}".to_string());
        match_templates.insert("config".to_string(), "*.{json,xml,yaml,yml,toml,ini}".to_string());
        match_templates.insert("no-extension".to_string(), "*.".to_string());
        match_templates.insert("hidden-files".to_string(), ".*".to_string());
        
        // Recursive patterns (all subdirectories)
        match_templates.insert("all-files-recursive".to_string(), "**/*.*".to_string());
        match_templates.insert("text-files-recursive".to_string(), "**/*.txt".to_string());
        match_templates.insert("images-recursive".to_string(), "**/*.{jpg,jpeg,png,gif,bmp,webp}".to_string());
        match_templates.insert("photos-recursive".to_string(), "**/*.{jpg,jpeg,png}".to_string());
        match_templates.insert("documents-recursive".to_string(), "**/*.{pdf,doc,docx,odt}".to_string());
        match_templates.insert("archives-recursive".to_string(), "**/*.{zip,tar,gz,bz2,7z,rar}".to_string());
        match_templates.insert("videos-recursive".to_string(), "**/*.{mp4,avi,mkv,mov,wmv,flv}".to_string());
        match_templates.insert("audio-recursive".to_string(), "**/*.{mp3,wav,flac,ogg,m4a}".to_string());
        match_templates.insert("code-recursive".to_string(), "**/*.{rs,py,js,ts,java,cpp,c,go}".to_string());
        match_templates.insert("scripts-recursive".to_string(), "**/*.{sh,bash,py,pl,rb}".to_string());
        match_templates.insert("config-recursive".to_string(), "**/*.{json,xml,yaml,yml,toml,ini}".to_string());
        
        // Exclusion pattern templates (filename-only patterns, no recursive needed)
        let mut exclusion_templates = HashMap::new();
        exclusion_templates.insert("all-files".to_string(), "*.*".to_string());
        exclusion_templates.insert("text-files".to_string(), "*.txt".to_string());
        exclusion_templates.insert("images".to_string(), "*.{jpg,jpeg,png,gif,bmp,webp}".to_string());
        exclusion_templates.insert("photos".to_string(), "*.{jpg,jpeg,png}".to_string());
        exclusion_templates.insert("documents".to_string(), "*.{pdf,doc,docx,odt}".to_string());
        exclusion_templates.insert("archives".to_string(), "*.{zip,tar,gz,bz2,7z,rar}".to_string());
        exclusion_templates.insert("videos".to_string(), "*.{mp4,avi,mkv,mov,wmv,flv}".to_string());
        exclusion_templates.insert("audio".to_string(), "*.{mp3,wav,flac,ogg,m4a}".to_string());
        exclusion_templates.insert("code".to_string(), "*.{rs,py,js,ts,java,cpp,c,go}".to_string());
        exclusion_templates.insert("scripts".to_string(), "*.{sh,bash,py,pl,rb}".to_string());
        exclusion_templates.insert("config".to_string(), "*.{json,xml,yaml,yml,toml,ini}".to_string());
        exclusion_templates.insert("no-extension".to_string(), "*.".to_string());
        exclusion_templates.insert("hidden-files".to_string(), ".*".to_string());
        
        Self {
            rename_templates,
            match_templates,
            exclusion_templates,
        }
    }
    
    /// List templates for a specific field type
    pub fn list_for_field(&self, field: TemplateField) -> Vec<(&String, &String)> {
        let templates = match field {
            TemplateField::MatchPattern => &self.match_templates,
            TemplateField::ExclusionPattern => &self.exclusion_templates,
            TemplateField::RenamingRule => &self.rename_templates,
        };
        let mut items: Vec<_> = templates.iter().collect();
        items.sort_by_key(|(k, _)| *k);
        items
    }
    
}

impl Default for TemplateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

