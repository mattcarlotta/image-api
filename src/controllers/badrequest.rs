use crate::http::{Request, Response};
use crate::utils::bad_req_file;

pub fn badrequest(_req: Request, res: Response) {
    res.set_status(400).send(bad_req_file())
}
