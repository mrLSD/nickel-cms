use super::*;
use middleware::form_validator::FormValidator;

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
    fn fill_form(form_data: &Params) -> Self {
        let mut done_c: Vec<String> = Vec::new();
        for v in form_data.all("done.c[]").unwrap() {
            done_c.push(v.to_owned());
        }
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
        PageForm {
            title: form_data.get("title").unwrap().parse().unwrap(),
            age: form_data.get("age").unwrap().parse().unwrap(),
            done: page_form_done,
            papers: page_form_papers,
        }
    }

}
