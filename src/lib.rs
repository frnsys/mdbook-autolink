use mdbook::{
    book::{Book, Chapter},
    errors::Error,
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};
use regex::Regex;
use std::{collections::HashMap, path::PathBuf};

pub struct AutoLink;

impl Preprocessor for AutoLink {
    fn name(&self) -> &str {
        "autolink-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let re = Regex::new(r"\[\[([^\]\|]+)(?:\|([^\]]+))?\]\]").unwrap();

        let chapters = book
            .iter()
            .filter_map(|item| {
                if let BookItem::Chapter(chapter) = item {
                    chapter.path.as_ref().map(|path| {
                        let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                        (name, path.to_path_buf())
                    })
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        // dbg!(&chapters);

        book.for_each_mut(|item| {
            process_section(item, &chapters, &re);
        });
        Ok(book)
    }
}

fn process_section(
    section: &mut BookItem,
    chapters: &HashMap<String, PathBuf>,
    link_regex: &Regex,
) {
    match section {
        BookItem::Chapter(Chapter {
            content, sub_items, ..
        }) => {
            let mut new_content = String::new();
            let mut in_code_block = false;

            for line in content.lines() {
                if line.trim_start().starts_with("```") {
                    // Toggle code block state
                    in_code_block = !in_code_block;
                    new_content.push_str(line);
                    new_content.push('\n');
                    continue;
                }

                if !in_code_block {
                    let mut processed_line = line.to_string();
                    for capture in link_regex.captures_iter(line) {
                        if let Some(link_text) = capture.get(1) {
                            let link_text = link_text.as_str();
                            if let Some(path) = chapters.get(link_text) {
                                processed_line = processed_line.replace(
                                    &format!("[[{}]]", link_text),
                                    &format!("[{}]({})", link_text, path.display()),
                                );
                            } else {
                                eprintln!("No chapter found for link {link_text:?}");
                            }
                        }
                    }
                    new_content.push_str(&processed_line);
                } else {
                    // Preserve lines inside code blocks
                    new_content.push_str(line);
                }
                new_content.push('\n');
            }

            *content = new_content;
            for sub_item in sub_items {
                process_section(sub_item, chapters, link_regex);
            }
        }
        _ => {}
    }
}
