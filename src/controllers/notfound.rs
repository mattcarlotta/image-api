use crate::http::{Request, Response};
use crate::utils::file_not_found;

pub fn notfound(_req: Request, mut res: Response) -> () {
    res.set_status(404);
    return res.send(file_not_found());
}
