
use crate::{cli::{YtVideoArgs, Config}, subtitle::fetch_video_info, common::print_info};

pub fn exec(args: YtVideoArgs, _config: Config) -> Result<(), anyhow::Error> {

    let info = fetch_video_info(&args.url)?;

    print_info(format!("Title: {}", info.name).as_str());

    if let Some(channel) = info.channel {
        print_info(format!("Channel: {}", channel).as_str());
    }

    for choice in info.choices {
        print_info(format!("Subtitle: {}", choice).as_str());
    }
    
    Ok(())
}
