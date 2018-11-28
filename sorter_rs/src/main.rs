use std::collections::HashMap;
use std::fs;
use std::env;
extern crate dirs;
#[allow(bad_style)]

fn identify() -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    //find files matching certain endings and add them
    //to the correct key of the HashMap (i.e. <"videos",["hrgl.mp4", "wrgl.mkv"]>)

    //this now fools the tests that are written (all up to ROM),
    //despite the fact that it only identifies tif type files

    let downloadPATH = dirs::download_dir().expect("failed to unwrap path");

    let filesInDownloads = fs::read_dir(&downloadPATH).expect("failed to read contents of download directory");

    for fileNAME in filesInDownloads {
        let entry = fileNAME.expect("DirEntry returned 0");
        let fileNAME: String = entry.file_name()
                                //this converts the OSstr into a string slice
                                .into_string()
                                .expect("the file_name could not be converted to a string")
                                //this converts the string slice into an owned string
                                .to_owned().clone();

        //the first one is a twofer cos it matches tiff and tif
        //following the || contains pattern, you can have all the image filetypes
        //use the same map insert code, instead of the bad example after this using else if
        if fileNAME.contains(".tif") ||
            fileNAME.contains(".gif") ||
            fileNAME.contains(".jpeg")
        {
            if !map.contains_key("Images") {
                            //this is the key,  this is the value
                map.insert("Images".to_owned(), vec![fileNAME]);
            } else {
                                    //this is the key
                let fileLIST = map.get_mut("Images").expect("failed to get mutable reference to Image list");
                //this is the value
                fileLIST.push(fileNAME);
            }
        //this is repetitive and thus bad
        } else if fileNAME.contains(".jpg") {
            if !map.contains_key("Images") {
                            //this is the key,  this is the value
                map.insert("Images".to_owned(), vec![fileNAME]);
            } else {
                                    //this is the key
                let fileLIST = map.get_mut("Images").expect("failed to get mutable reference to executable list");
                //this is the value
                fileLIST.push(fileNAME);
            }
        } else {
            //this branch is hit when the file found in downloads doesnt match any of the types so far
            //when you use continue, it skips this file and moves on to the next one
            continue
        }


        //if file_name contains "jpg"
        //if file_name contains "jfif"
        //etc
        //etc


    }


    return map;
}

fn create_dirs(map: &HashMap<String, Vec<String>>){

}

fn move_files(map: &HashMap<String, Vec<String>>){

}


fn main() {
    let map = identify();
    create_dirs(&map);
    move_files(&map);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identify_returns_map_of_image_files() -> Result<(), ()> {
        let file_list = vec!["brgl.tif",
                            "drgl.tiff",
                            "frgl.gif",
                            "grgl.jpeg",
                            "hrgl.jpg",
                            "jrgl.jif",
                            "krgl.jfif",
                            "lrgl.jp2",
                            "mrgl.jpx",
                            "nrgl.j2k",
                            "prgl.j2c",
                            "srgl.fpx",
                            "trgl.pcd",
                            "urgl.png",
                            "vrgl.pdf"];

        let category = "Images".to_string();

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set the current dir to Downloads");

        for file in &file_list {
            let created_file = fs::File::create(&file).expect("failed to create file");
            created_file.sync_all().expect("failed to be sure the file was created");
        }

        println!("\nthis is what the identify function returns when given image file types: \n{:?}\n", identify());

        let mut err_count = 0;
        if identify().len() == 0 {
            for file in &file_list{
                fs::remove_file(&file).expect("failed to remove file without match");
                let index = file.chars().position(|letter| letter == '.').expect("failed to find '.' in filename");
                println!("the filetype missed was {}", &file[index..]);
                err_count += 1;
            }
        }

        for value in identify().values(){
            for file in &file_list {
                if value.contains(&file.to_string()){
                    fs::remove_file(&file).expect("failed to remove file after match");
                } else {
                    fs::remove_file(&file).expect("failed to remove file without match");
                    let index = file.chars().position(|letter| letter == '.').expect("failed to find '.' in filename");
                    println!("the filetype missed was {}", &file[index..]);
                    err_count += 1;
                }
            }
        }

        for key in identify().keys(){
            if key.contains(&category){
                continue;
            } else {
                println!("the category key missed was {}", &category);
                err_count += 1;
            }
        }

        if err_count > 0 {
            return Err(());
        } else {
            return Ok(());
        }
    }

    fn identify_returns_map_of_executable_files() -> Result<(), ()> {
        let file_list = vec!["brgl.exe",
                            "drgl.dmg",
                            "frgl.AppImage",
                            "grgl",
                            "hrgl.deb",
                            "jrgl.rpm",
                            "krgl.pkg",
                            "lrgl.msi"];

        let category = "Executables".to_string();

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set current dir to Downloads");

        for file in &file_list {
            let created_file = fs::File::create(&file).expect("failed to create file");
            created_file.sync_all().expect("failed to be sure the file was created");
        }

        println!("\nthis is what the identify function returns when given executable file types: \n{:?}\n", identify());

        let mut err_count = 0;
        if identify().len() == 0 {
            for file in &file_list{
                fs::remove_file(&file).expect("failed to remove file");
                let mut in_box = file.clone().to_owned();
                let out_box = match in_box.chars().position(|letter| letter == '.'){
                    Some(i) => in_box[i..].to_owned(),
                    None => format!("unix executable ({})", in_box).to_owned()
                };
                println!("the filetype missed was {}", &out_box);
                err_count += 1;
            }
        }

        for value in identify().values(){
            for file in &file_list {
                if value.contains(&file.to_string()){
                    fs::remove_file(&file).expect("failed to remove file");
                } else {
                    fs::remove_file(&file).expect("failed to remove file");
                    let mut in_box = file.clone().to_owned();
                    let out_box = match in_box.chars().position(|letter| letter == '.'){
                        Some(i) => in_box[i..].to_owned(),
                        None => in_box[..].to_owned()
                    };
                    println!("the filetype missed was {}", &out_box);
                    err_count += 1;
                }
            }
        }
        for key in identify().keys(){
            if key.contains(&category){
                continue;
            } else {
                println!("the category key missed was {}", &category);
                err_count += 1;
            }
        }

        if err_count > 0 {
            return Err(());
        } else {
            return Ok(());
        }
    }

    fn identify_returns_map_of_video_files() -> Result<(), ()> {
        let file_list = vec!["brgl.asf",
                            "drgl.wma",
                            "frgl.wmv",
                            "grgl.mp4",
                            "hrgl.m4a",
                            "jrgl.m4v",
                            "krgl.f4v",
                            "lrgl.f4a",
                            "mrgl.m4b",
                            "nrgl.mov",
                            "prgl.3gp",
                            "srgl.3gp2",
                            "trgl.3g2",
                            "urgl.3gpp",
                            "vrgl.3gpp2",
                            "brgl.ogg",
                            "drgl.oga",
                            "frgl.ogv",
                            "grgl.ogx",
                            "hrgl.webm",
                            "jrgl.flv",
                            "krgl.avi"];

        let category = "Videos".to_string();

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set current dir to Downloads");

        for file in &file_list {
            let created_file = fs::File::create(&file).expect("failed to create file");
            created_file.sync_all().expect("failed to be sure the file was created");
        }
        println!("\nthis is what the identify function returns when given video file types: \n{:?}\n", identify());

        let mut err_count = 0;
        if identify().len() == 0 {
            for file in &file_list{
                fs::remove_file(&file).expect("failed to remove file without match");
                let index = file.chars().position(|letter| letter == '.').expect("failed to find '.' in filename");
                println!("the filetype missed was {}", &file[index..]);
                err_count += 1;
            }
        }

        for value in identify().values(){
            for file in &file_list {
                if value.contains(&file.to_string()){
                    fs::remove_file(&file).expect("failed to remove file");
                } else {
                    fs::remove_file(&file).expect("failed to remove file");
                    let index = file.chars().position(|letter| letter == '.').expect("failed to find '.' in file name");
                    println!("the filetype missed was {}", &file[index..]);
                    err_count += 1;
                }
            }
        }
        for key in identify().keys(){
            if key.contains(&category){
                continue;
            } else {
                println!("the category key missed was {}", &category);
                err_count += 1;
            }
        }

        if err_count > 0 {
            return Err(());
        } else {
            return Ok(());
        }
    }

    fn identify_returns_map_of_rom_files() -> Result<(), ()> {
        let file_list = vec!["brgl.iso",
                            "drgl.bin",
                            "frgl.img",
                            "grgl.vdi",
                            "hrgl.rom"];

        let category = "Roms".to_string();

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set current dir to Downloads");

        for file in &file_list {
            let created_file = fs::File::create(&file).expect("failed to create file");
            created_file.sync_all().expect("failed to be sure the file was created");
        }
        println!("\nthis is what the identify function returns when given disk file types: \n{:?}\n", identify());

        let mut err_count = 0;
        if identify().len() == 0 {
            for file in &file_list{
                fs::remove_file(&file).expect("failed to remove file without match");
                let index = file.chars().position(|letter| letter == '.').expect("failed to find '.' in filename");
                println!("the filetype missed was {}", &file[index..]);
                err_count += 1;
            }
        }

        for value in identify().values(){
            for file in &file_list {
                if value.contains(&file.to_string()){
                    fs::remove_file(&file).expect("failed to remove file");
                } else {
                    fs::remove_file(&file).expect("failed to remove file");
                    let index = file.chars().position(|letter| letter == '.').expect("failed to find '.' in file name");
                    println!("the filetype missed was {}", &file[index..]);
                    err_count += 1;

                }
            }
        }
        for key in identify().keys(){
            if key.contains(&category){
                continue;
            } else {
                println!("the category key missed was {}", &category);
                err_count += 1;
            }
        }

        if err_count > 0 {
            return Err(());
        } else {
            return Ok(());
        }
    }

    fn identify_returns_map_of_compressed_files() -> Result<(),()> {
        let file_list = vec!["brgl.7z",
                            "drgl.bz",
                            "frgl.rar",
                            "grgl.tar",
                            "hrgl.zip"];

        let category = "Compressed".to_string();

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set current dir to Downloads");

        for file in &file_list {
            let created_file = fs::File::create(&file).expect("failed to create file");
            created_file.sync_all().expect("failed to be sure the file was created");
        }

        println!("\nthis is what the identify function returns when given compressed file types: \n{:?}\n", identify());

        let mut err_count = 0;
        if identify().len() == 0 {
            for file in &file_list{
                fs::remove_file(&file).expect("failed to remove file without match");
                let index = file.chars().position(|letter| letter == '.').expect("failed to find '.' in filename");
                println!("the filetype missed was {}", &file[index..]);
                err_count += 1;
            }
        }

        for value in identify().values(){
            for file in &file_list {
                if value.contains(&file.to_string()){
                    fs::remove_file(&file).expect("failed to remove file");
                } else {
                    fs::remove_file(&file).expect("failed to remove file");
                    let index = file.chars().position(|letter| letter == '.').expect("failed to find '.' in file name");
                    println!("the filetype missed was {}", &file[index..]);
                    err_count += 1;
                }
            }
        }
        for key in identify().keys(){
            if key.contains(&category){
                continue;
            } else {
                println!("the category key missed was {}", &category);
                err_count += 1;
            }
        }

        if err_count > 0 {
            return Err(());
        } else {
            return Ok(());
        }
    }

    fn identify_returns_map_of_torrent_files() -> Result<(), ()> {
        let file_list = vec!["brgl.torrent"];

        let category = "Torrents".to_string();

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set current dir to Downloads");

        for file in &file_list {
            let created_file = fs::File::create(&file).expect("failed to create file");
            created_file.sync_all().expect("failed to be sure the file was created");
        }
        println!("\nthis is what the identify function returns when given the torrent file type: \n{:?}\n", identify());

        let mut err_count = 0;
        if identify().len() == 0 {
            for file in &file_list{
                fs::remove_file(&file).expect("failed to remove file without match");
                let index = file.chars().position(|letter| letter == '.').expect("failed to find '.' in filename");
                println!("the filetype missed was {}", &file[index..]);
                err_count += 1;
            }
        }

        for value in identify().values(){
            for file in &file_list {
                if value.contains(&file.to_string()){
                    fs::remove_file(&file).expect("failed to remove file");
                } else {
                    fs::remove_file(&file).expect("failed to remove file");
                    let index = file.chars().position(|letter| letter == '.').expect("failed to find '.' in file name");
                    println!("the filetype missed was {}", &file[index..]);
                    err_count += 1;
                }
            }
        }
        for key in identify().keys(){
            if key.contains(&category){
                continue;
            } else {
                println!("the category key missed was {}", &category);
                err_count += 1;
            }
        }

        if err_count > 0 {
            return Err(())
        } else {
            return Ok(())
        }
    }

    fn identify_returns_map_of_multiple_types() -> Result<(), ()> {
        let mut file_map = HashMap::new();
        file_map.insert("Compressed".to_string(), vec!["brgl.7z".to_string()]);
        file_map.insert("Torrents".to_string(), vec!["brgl.torrent".to_string()]);
        file_map.insert("Roms".to_string(), vec!["brgl.iso".to_string()]);
        file_map.insert("Videos".to_string(), vec!["brgl.asf".to_string()]);
        file_map.insert("Executables".to_string(), vec!["brgl.exe".to_string()]);
        file_map.insert("Images".to_string(), vec!["brgl.tif".to_string()]);

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set current dir to Downloads");

        for file in file_map.values() {
            let created_file = fs::File::create(&file[0]).expect("failed to create file");
            created_file.sync_all().expect("failed to be sure the file was created");
        }

        println!("\nthis is what the identify function returns when given multiple file types: \n{:?}\n", identify());

        if file_map == identify() {
            return Ok(())
        } else {
            return Err(())
        }
    }

    fn identify_ignores_file_names_containing_extensions() -> Result<(),()> {
        let file_list = vec!["torrent.bad",
                            "zippy.bad",
                            "isolated.bad",
                            "movies.bad",
                            "debian.bad",
                            "jiffylube.bad"];

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set current dir to downloads");

        for file in &file_list {
            let created_file = fs::File::create(&file).expect("failed to create file");
            created_file.sync_all().expect("failed to be sure the file was created");
        }

        println!("\nthis is what the identify function returns when given bad filenames: \n{:?}\n", identify());

        let mut err_count = 0;
        if identify().len() == 0 {
            for file in &file_list{
                fs::remove_file(&file).expect("failed to remove file without match");
                err_count += 1;
            }
        }

        for value in identify().values(){
            for file in &file_list {
                if value.contains(&file.to_string()){
                    fs::remove_file(&file).expect("failed to remove file");
                    println!("should not have identified {}", &file);
                    err_count += 1;
                } else {
                    fs::remove_file(&file).expect("failed to remove file");
                }
            }
        }
        if err_count > 0 {
            return Err(())
        } else {
            return Ok(())
        }
    }

    #[test]
    fn identify_returns_correct_maps(){
        identify_returns_map_of_image_files().expect("the output of identify did not match the image files in the downloads folder");
        identify_returns_map_of_executable_files().expect("the output of identify did not match the executable files in the downloads folder");
        identify_returns_map_of_video_files().expect("the output of identify did not match the video files in the downloads folder");
        identify_returns_map_of_rom_files().expect("the output of identify did not match the rom files in the downloads folder");
        identify_returns_map_of_compressed_files().expect("the output of identify did not match the compressed files in the downloads folder");
        identify_returns_map_of_torrent_files().expect("the output of identify did not match the compressed files in the downloads folder");

        identify_ignores_file_names_containing_extensions().expect("the identify function was fooled by file_names containing extensions");
        identify_returns_map_of_multiple_types().expect("the output of identify did not match the multiple types in the downloads folder");
    }

    #[test]
    fn creates_dirs_from_map_keys(){
        let mut file_map = HashMap::new();
        file_map.insert("Compressed".to_string(), vec!["brgl.7z".to_string()]);
        file_map.insert("Torrents".to_string(), vec!["brgl.torrent".to_string()]);
        file_map.insert("Roms".to_string(), vec!["brgl.iso".to_string()]);
        file_map.insert("Videos".to_string(), vec!["brgl.asf".to_string()]);
        file_map.insert("Executables".to_string(), vec!["brgl.exe".to_string()]);
        file_map.insert("Images".to_string(), vec!["brgl.tif".to_string()]);

        let mut missed_list: Vec<String> = vec![];

        let download_path = dirs::download_dir().expect("failed to unwrap path");

        create_dirs(&file_map);

        let mut ok_count = 0;
        for entry in fs::read_dir(&download_path).expect("failed to read directory contents") {
            let entry = entry.expect("could not get a dir_name from the DirEntry");
            let dir_name = &entry.file_name()
                                .into_string()
                                .expect("could not convert dir_name to string")
                                .to_owned();

            for key in file_map.keys() {
                if dir_name.contains(&key[..]) {
                    fs::remove_dir_all(&key).expect("failed to remove directory");
                    ok_count += 1;
                } else {
                    if !missed_list.contains(&key){
                        missed_list.push(key.to_owned());
                    }
                    continue;
                }
            }
        }
        if ok_count == file_map.keys().len() {
            assert!(true);
        } else {
            for key in missed_list{
                println!("the {} directory was not created by create_dirs()", &key);
            }
            panic!("only {} out of {} directories were successfully created", ok_count, file_map.keys().len());
        }
    }

    fn files_from_map_in_correct_dirs() -> Result<(), ()> {
        let mut file_map = HashMap::new();
        file_map.insert("Compressed".to_string(), vec!["brgl.7z".to_string()]);
        file_map.insert("Torrents".to_string(), vec!["brgl.torrent".to_string()]);
        file_map.insert("Roms".to_string(), vec!["brgl.iso".to_string()]);
        file_map.insert("Videos".to_string(), vec!["brgl.asf".to_string()]);
        file_map.insert("Executables".to_string(), vec!["brgl.exe".to_string()]);
        file_map.insert("Images".to_string(), vec!["brgl.tif".to_string()]);

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set current dir to Downloads");

        for directory in file_map.keys() {
            fs::create_dir_all(&directory).expect("failed to create folder");
        }

        for file in file_map.values() {
            let created_file = fs::File::create(&file[0]).expect("failed to create file");
            created_file.sync_all().expect("failed to be sure the file was created");
        }

        move_files(&file_map);

        let mut ok_count = 0;
        'dir: for entry in fs::read_dir(&download_path).expect("failed to get file list from directory") {
            let entry = entry.expect("failed to unwrap entry");
            let entry_name = &entry.file_name().into_string().expect("failed to convert dir entry to string");
            for key in file_map.keys() {
                if key.contains(&entry_name[..]) {
                    env::set_current_dir(&entry.path()).expect("failed to set map key dir");
                    'file: for file in fs::read_dir(&entry.path()).expect("failed to get file list from directory") {
                        let file = file.expect("failed to unwrap entry");
                        let file_name = &file.file_name().into_string().expect("failed to convert dir entry to string");

                        if file_name.contains(&file_map[key][0][..]){
                            ok_count += 1;
                        } else {
                            continue 'dir
                        }
                    }
                }
            }
        }

        let download_path = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&download_path).expect("failed to set current dir to Downloads");

        for directory in file_map.keys() {
            fs::remove_dir_all(&directory).expect("failed to remove folder");
        }

        if ok_count != 6 {
            for file in file_map.values() {
                match fs::remove_file(&file[0]){
                    Ok(_) => continue,
                    Err(_) => continue
                }
            }
        }

        if ok_count == 6 {
            return Ok(());
        } else {
            return Err(());
        }
    }

    fn files_moved_no_longer_in_downloads() -> Result<(), ()> {
        let mut file_map = HashMap::new();
        file_map.insert("Compressed".to_string(), vec!["brgl.7z".to_string()]);
        file_map.insert("Torrents".to_string(), vec!["brgl.torrent".to_string()]);
        file_map.insert("Roms".to_string(), vec!["brgl.iso".to_string()]);
        file_map.insert("Videos".to_string(), vec!["brgl.asf".to_string()]);
        file_map.insert("Executables".to_string(), vec!["brgl.exe".to_string()]);
        file_map.insert("Images".to_string(), vec!["brgl.tif".to_string()]);

        let download_path = dirs::download_dir().expect("failed to unwrap path");

        let mut err_count = 0;
        for file in fs::read_dir(&download_path).expect("failed to read contents of downloads"){
            let file = file.expect("failed to unwrap dir entry");
            let file_name = &file.file_name().into_string().expect("failed to convert dir entry to string");
            for value in file_map.values(){
                if value.contains(file_name) {
                    println!("move_files() did not remove the {} file after copying", file_name);
                    fs::remove_file(file.path()).expect("failed to remove file");
                    err_count += 1;
                } else {
                    continue;
                }
            }
        }
        if err_count > 0 {
            return Err(());
        } else {
            return Ok(());
        }
    }

    #[test]
    fn copies_files_and_cleans_up() {
        files_from_map_in_correct_dirs().expect("the files did not end up in the correct dirs after move");
        files_moved_no_longer_in_downloads().expect("the files were still in the download folder after move");
    }
}
