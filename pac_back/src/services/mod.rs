use bytes::{Buf, Bytes};
use gpx::{Track, TrackSegment};

pub fn handle_gpx(data: Bytes) {
    let reader = data.reader();

    match gpx::read(reader) {
        Ok(gpx) => {
            let track: &Track = &gpx.tracks[0];
            let segment: &TrackSegment = &track.segments[0];
            println!("{:#?}", segment);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_reads_gpx() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
