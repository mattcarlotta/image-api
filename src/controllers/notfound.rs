use crate::http::{file_not_found, Request, Response};

pub fn notfound(_req: Request, mut res: Response) -> () {
    res.set_status(404);
    return res.send(file_not_found());
}
