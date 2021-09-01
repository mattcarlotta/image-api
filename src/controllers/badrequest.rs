use crate::http::{bad_req_file, Request, Response};

pub fn badrequest(_req: Request, mut res: Response) -> () {
    res.set_status(400);
    return res.send(bad_req_file());
}
