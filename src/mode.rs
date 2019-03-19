pub fn format_mode(mode: u32) -> String{

    let mut perms = String::from("");

    if (mode & (0x1<<9)) >= 1 { perms.push('d') } else { perms.push('-') };
    if (mode & (0x1<<8)) >= 1 { perms.push('r') } else { perms.push('-') };
    if (mode & (0x1<<7)) >= 1 { perms.push('w') } else { perms.push('-') };
    if (mode & (0x1<<6)) >= 1 { perms.push('x') } else { perms.push('-') };
    if (mode & (0x1<<5)) >= 1 { perms.push('r') } else { perms.push('-') };
    if (mode & (0x1<<4)) >= 1 { perms.push('w') } else { perms.push('-') };
    if (mode & (0x1<<3)) >= 1 { perms.push('x') } else { perms.push('-') };
    if (mode & (0x1<<2)) >= 1 { perms.push('r') } else { perms.push('-') };
    if (mode & (0x1<<1)) >= 1 { perms.push('w') } else { perms.push('-') };
    if (mode & 0x1) >= 1 { perms.push('x') } else { perms.push('-') };

    perms
}
