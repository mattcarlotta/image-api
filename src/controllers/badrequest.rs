use crate::http::{Request, Response};
use crate::utils::bad_req_file;

pub fn badrequest(_req: Request, mut res: Response) -> () {
    res.set_status(400);
    return res.send(bad_req_file());
}
