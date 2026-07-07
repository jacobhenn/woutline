use std::{env, fmt::Write, fs, path::Path};

use lopdf::{Bookmark, Document, Object};

use anyhow::{Context, Result, bail};

fn read(args: &mut env::Args) -> Result<()> {
    let document_path_str = args
        .next()
        .context("expected argument <document>; see --help")?;
    let document_path = Path::new(&document_path_str);
    let outline_path = args.next().unwrap_or_else(|| {
        format!(
            "{}~OUTLINE.txt",
            document_path.file_prefix().unwrap().to_string_lossy()
        )
    });
    if let Some(arg) = args.next() {
        bail!("unexpected argument '{arg}'; see --help");
    }

    let document = Document::load(document_path).context("loading document")?;

    let mut outline_buf = String::new();

    let toc = document.get_toc().context("getting document TOC")?;
    for toc_entry in toc.toc.iter() {
        for _ in 0..(toc_entry.level - 1) * 4 {
            outline_buf.push(' ');
        }
        writeln!(&mut outline_buf, "{} {}", toc_entry.page, toc_entry.title)
            .expect("enough memory to build the outline string");
    }

    fs::write(&outline_path, outline_buf)
        .with_context(|| format!("writing outline to {outline_path}"))?;

    println!("wrote {} TOC entries to {}", toc.toc.len(), outline_path);

    Ok(())
}

fn write(args: &mut env::Args) -> Result<()> {
    let document_path_str = args
        .next()
        .context("expected argument <document>; see --help")?;
    let document_path = Path::new(&document_path_str);
    let outline_path = args.next().unwrap_or_else(|| {
        format!(
            "{}~OUTLINE.txt",
            document_path.file_prefix().unwrap().to_string_lossy()
        )
    });
    if let Some(arg) = args.next() {
        bail!("unexpected argument '{arg}'; see --help");
    }

    let mut document = Document::load(document_path).context("loading document")?;
    let pages = document.get_pages();

    let outline_str =
        fs::read_to_string(&outline_path).with_context(|| format!("reading {outline_path:?}"))?;

    let mut parent_stack = Vec::new();
    let mut prev_level = 0;
    let mut prev_bookmark_id = 0;
    for (line_number, line) in outline_str.lines().enumerate() {
        let rest = line.trim_start();
        let new_level = (line.len() - rest.len()) / 4;
        if new_level < prev_level {
            parent_stack.pop();
        } else if new_level > prev_level {
            parent_stack.push(prev_bookmark_id);
        }
        let (page_number_str, title) = rest.split_once(' ').with_context(|| format!("reading outline at {outline_path}: expected at least one space on line {line_number} to delimit page number from title"))?;
        let page_number: u32 = page_number_str.parse().with_context(|| {
            format!("reading outline at {outline_path}: '{page_number_str}' is not an integer")
        })?;
        let page_id = *pages.get(&page_number).with_context(|| {
            format!("loading outline at {outline_path}: document does not have page {page_number}")
        })?;
        let bookmark = Bookmark {
            children: Vec::new(),
            title: String::from(title),
            format: 0,
            color: [0.0; 3],
            page: page_id,
            id: 0,
        };
        let parent = parent_stack.last().copied();
        let bookmark_id = document.add_bookmark(bookmark, parent);
        prev_level = new_level;
        prev_bookmark_id = bookmark_id;
    }

    let outline_id = document
        .build_outline()
        .context("building outline of document")?;
    let catalog = document.catalog_mut().context("getting document catalog")?;
    catalog.set("Outlines", Object::Reference(outline_id));

    document
        .save(document_path)
        .with_context(|| format!("saving new document to {document_path:?}"))?;

    Ok(())
}

fn help(args: &mut env::Args) -> Result<()> {
    if let Some(arg) = args.next() {
        bail!("unexpected argument '{arg}'; see --help");
    }

    println!("Usage:
  woutline r <document> [outline]   Read the outline from <document> and write it to [outline] (default: <document-name>~OUTLINE.txt)
  woutline w <document> [outline]   Read the outline from [outline] (default: <document-name>~OUTLINE.txt) and write it to <document>

For example, `woutline r test.pdf` will write the outline to `test~OUTLINE.txt`.
  
Currently, woutline only supports PDFs. `woutline w` should preserve the content of <document>, but may not preserve how that content is stored. To restore a property like compression or use of object streams, maybe use qpdf.");

    Ok(())
}

fn main() -> Result<()> {
    let mut args = env::args();
    args.next().expect("at least one OS-supplied argument");
    let subcommand_str = args.next().context("expected subcommand; see --help")?;
    match subcommand_str.as_str() {
        "r" => read(&mut args)?,
        "w" => write(&mut args)?,
        "--help" => help(&mut args)?,
        other => bail!("unrecognized subcommand '{other}'; see --help"),
    };

    Ok(())
}
