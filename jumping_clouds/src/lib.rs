pub fn jump_on_clouds(thunderheads: &Vec<u32>) -> u32 {
    let mut hops = vec![0; thunderheads.len()];

    if thunderheads.len() < 2 {
        return 0;
    }

    for i in (0..(hops.len() - 2)).rev() {
        if thunderheads[i] == 1 {
            continue;
        }
        let mut options: Vec<u32> = Vec::new();
        if i + 2 > hops.len() - 1 || thunderheads[i + 2] != 1 {
            options.push(hops[i + 2])
        }
        if i + 1 > hops.len() - 1 || thunderheads[i + 1] != 1 {
            options.push(hops[i + 1])
        }

        options.sort();
        hops[i] = options[0] + 1;
    }

    hops[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_cloud_has_zero_steps() {
        let c = vec![0];
        let actual = jump_on_clouds(&c);
        assert_eq!(0, actual)
    }

    #[test]
    fn two_clouds_has_single_step() {
        let c = vec![0; 3];
        let actual = jump_on_clouds(&c);
        assert_eq!(1, actual)
    }
    #[test]
    fn longer_path() {
        let c = vec![0, 0, 0, 0, 1, 0];
        let actual = jump_on_clouds(&c);
        assert_eq!(3, actual)
    }

    #[test]
    fn multiple_hops() {
        let c = vec![0, 0, 1, 0, 0, 1, 0];
        let actual = jump_on_clouds(&c);
        assert_eq!(4, actual)
    }
}
