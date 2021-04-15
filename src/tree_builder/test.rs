use crate::{
    size::Bytes,
    tree_builder::{Info, Tree, TreeBuilder},
};
use build_fs_tree::{dir, file, FileSystemTree};
use derive_more::From;
use pretty_assertions::assert_eq;

type SampleData = Bytes;
type SampleId = String;
const SAMPLE_SEPARATOR: char = '/';
const SAMPLE_DIR_SIZE: SampleData = Bytes::new(5);

#[derive(Debug, From)]
struct SampleTree(FileSystemTree<String, &'static str>);

const fn len(text: &str) -> SampleData {
    SampleData::new(text.len() as u64)
}

impl SampleTree {
    fn create_sample() -> Self {
        SampleTree::from(dir! {
            "flat" => dir! {
                "0" => file!("")
                "1" => file!("a")
                "2" => file!("ab")
                "3" => file!("abc")
            }
            "nested" => dir! {
                "0" => dir! {
                    "1" => file!("abcdef")
                }
            }
            "empty-dir" => dir! {}
        })
    }

    fn tree(&self, id: SampleId) -> Tree<SampleId, SampleData> {
        Tree::from(TreeBuilder {
            id,
            get_info: |path| {
                let path: Vec<_> = path
                    .split(SAMPLE_SEPARATOR)
                    .map(ToString::to_string)
                    .collect();
                let mut path = path.iter();
                match self.0.path(&mut path) {
                    Some(FileSystemTree::File(content)) => Info::from((len(content), Vec::new())),
                    Some(FileSystemTree::Directory(content)) => Info::from((
                        SAMPLE_DIR_SIZE,
                        content.keys().map(ToString::to_string).collect(),
                    )),
                    None => panic!("Path does not exist"),
                }
            },
            join_path: |prefix, name| format!("{}{}{}", prefix, SAMPLE_SEPARATOR, name),
        })
    }
}

#[test]
fn flat() {
    let actual = SampleTree::create_sample().tree("flat".to_string());
    let expected = Tree {
        id: "flat".to_string(),
        data: len("") + len("a") + len("ab") + len("abc") + SAMPLE_DIR_SIZE,
        children: vec![
            Tree {
                id: "flat/0".to_string(),
                data: len(""),
                children: Vec::new(),
            },
            Tree {
                id: "flat/1".to_string(),
                data: len("a"),
                children: Vec::new(),
            },
            Tree {
                id: "flat/2".to_string(),
                data: len("ab"),
                children: Vec::new(),
            },
            Tree {
                id: "flat/3".to_string(),
                data: len("abc"),
                children: Vec::new(),
            },
        ],
    };
    assert_eq!(actual, expected);
}

#[test]
fn nested() {
    let actual = SampleTree::create_sample().tree("nested".to_string());
    let expected = Tree {
        id: "nested".to_string(),
        data: len("abcdef") + SAMPLE_DIR_SIZE + SAMPLE_DIR_SIZE,
        children: vec![Tree {
            id: "nested/0".to_string(),
            data: len("abcdef") + SAMPLE_DIR_SIZE,
            children: vec![Tree {
                id: "nested/0/1".to_string(),
                data: len("abcdef"),
                children: Vec::new(),
            }],
        }],
    };
    assert_eq!(actual, expected);
}

#[test]
fn empty_dir() {
    let actual = SampleTree::create_sample().tree("empty-dir".to_string());
    let expected = Tree {
        id: "empty-dir".to_string(),
        data: SAMPLE_DIR_SIZE,
        children: Vec::new(),
    };
    assert_eq!(actual, expected);
}