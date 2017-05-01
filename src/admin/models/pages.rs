use super::*;
use middleware::form_validator::*;

#[derive(Debug)]
pub struct PageForm {
    pub title: String,
    pub age: u8,
    pub done: PageFormDone,
    pub papers: Vec<PageFormPapers>,
}

#[derive(Debug)]
pub struct PageFormDone {
    pub a: String,
    pub b: f32,
    pub c: Vec<String>,
}

#[derive(Debug)]
pub struct PageFormPapers {
    pub title: String,
    pub pages: u16,
}

impl FormValidator for PageForm {
    fn validators<'a>() -> ValidatorParams<'a> {
        println!("Init validators");
        validators! (
            ["data.a", "tst2"]: f32 => required(), max(1);
            ["tst3", "tst4"]: i32 => max(1, "2+");
        )
    }

    fn fill_form(form_data: &Params) -> Result<Self, FormValidationErrors> {
        let mut done_c: Vec<String> = Vec::new();
        for v in form_data.all("done.c[]").unwrap() {
            done_c.push(v.to_owned());
        }
        let _t: String = form_data.get("done.a").unwrap().parse().unwrap();
        let page_form_done = PageFormDone {
            a: form_data.get("done.a").unwrap().parse().unwrap(),
            b: form_data.get("done.b").unwrap().parse().unwrap(),
            c: done_c,
        };
        let mut page_form_papers: Vec<PageFormPapers> = Vec::new();
        let papers_pages = form_data.all("papers[].pages").unwrap();
        for (i, v) in form_data.all("papers[].title").unwrap().iter().enumerate() {
            page_form_papers.push(PageFormPapers {
                title: v.to_owned(),
                pages: papers_pages[i].parse().unwrap(),
            })
        }
        Ok(PageForm {
            title: form_data.get("title").unwrap().parse().unwrap(),
            age: form_data.get("age").unwrap().parse().unwrap(),
            done: page_form_done,
            papers: page_form_papers,
        })
    }

}
