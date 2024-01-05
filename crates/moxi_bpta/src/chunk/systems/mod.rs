mod misc;
mod spawn;
mod update;

pub(crate) use spawn::*;
pub(crate) use update::*;

#[cfg(test)]
mod test {

    #[test]
    fn test_systems() {}
}
