use object::{Object, ObjectSection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let binary_data = std::fs::read(&args[1])?;
    let file = object::File::parse(&*binary_data)?;

    println!(
        "Format: {:?} {:?}-endian {}-bit",
        file.format(),
        file.endianness(),
        if file.is_64() { "64" } else { "32" }
    );
    println!("Kind: {:?}", file.kind());
    println!("Architecture: {:?}", file.architecture());
    if let Some(sub_architecture) = file.sub_architecture() {
        println!("Sub-Architecture: {:?}", sub_architecture);
    }

    let bitness = match file.architecture() {
        object::Architecture::X86_64 => 64,
        object::Architecture::X86_64_X32 => 32,
        object::Architecture::I386 => 32,
        _ => {
            eprintln!("Unsupported architecture: {:?}", file.architecture());
            std::process::exit(1);
        }
    };

    let mut iset_counter = vec![0usize; 256];
    let mut total_ins = 0;

    for section in file.sections() {
        if section.kind() != object::SectionKind::Text {
            continue;
        }

        let data = section.data()?;
        let decoder =
            iced_x86::Decoder::new(bitness, data, iced_x86::DecoderOptions::NO_INVALID_CHECK);
        for instruction in decoder {
            if instruction.is_invalid() {
                continue;
            }
            for &feature in instruction.cpuid_features() {
                iset_counter[feature as usize] += 1;
            }
            total_ins += 1;
        }
    }

    // Sort by count
    let mut ordered_iset_counter = iset_counter
        .into_iter()
        .enumerate()
        .filter(|&(_, count)| count != 0)
        .collect::<Vec<_>>();

    println!();
    println!("Instruction set usage (total {}):", total_ins);
    ordered_iset_counter.sort_by(|a, b| b.1.cmp(&a.1));
    for (iset, count) in ordered_iset_counter.iter() {
        let iset_e = iced_x86::CpuidFeature::try_from(*iset).unwrap();
        let percent = (*count as f64 / total_ins as f64) * 100.0;
        println!("{:?}: {} ({:.2}%)", iset_e, count, percent);
    }

    Ok(())
}
