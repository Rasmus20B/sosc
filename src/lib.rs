mod voiceline;
mod event;
use crate::voiceline::*;
use std::path::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voicelines() {
        let path = Path::new("assets/vls.yaml");
        let s : Vec<Voiceline> = vec!();
        let mut storage = SoundStorage {
            store : s
        };
        let vls_result = &voiceline::store_segments(path, &mut storage).unwrap();
        // assert_eq!(vls_result.store[0].id, 1);

        let ans = generate_voiceline(&vls_result, &event::Event { name: "goal".to_string(), id: 1, priority: 1 });

        println!("{}", ans);
    }
}
