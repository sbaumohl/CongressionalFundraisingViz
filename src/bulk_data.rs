pub mod fec_data_handler {
    use crate::entities::*;
    use sea_orm::ActiveValue;
    use std::{
        fs::{self, File},
        io::{self, BufRead},
        path::{Path, PathBuf},
    };

    // sorts paths to files  by file name in said folder. This is important for when data about the an entity changes year over year. Therefore we apply the newest data last, overwriting older data
    fn get_sorted_path_bufs(folder: &str) -> Vec<PathBuf> {
        let root_path = Path::new("./src/bin/").join(folder);
        let paths = fs::read_dir(root_path).unwrap();

        let mut sorted_paths: Vec<PathBuf> = paths
            .map(|path| path.expect("Invalid Path").path())
            .collect();

        sorted_paths.sort_by(|a, b| a.file_name().cmp(&b.file_name().to_owned()));

        return sorted_paths;
    }

    fn none_if_empty(x: &str) -> Option<String> {
        if x.eq("") {
            None
        } else {
            Some(x.to_string())
        }
    }

    // Data from: https://www.fec.gov/data/browse-data/?tab=bulk-data
    pub fn parse_bulk_committees() -> Vec<committees::ActiveModel> {
        // Data schema: https://www.fec.gov/campaign-finance-data/committee-master-file-description/

        let paths = get_sorted_path_bufs("committees/");

        let mut committees: Vec<committees::ActiveModel> = Vec::new();

        // each path contains bulk data for a single election cycle (2 years)
        // committee id's are unique, and the committee may rebrand, so I take the newest data first
        // to ensure that I have the most recent committee info

        for path in paths {
            let file = File::open(&path).expect("error reading file");
            let lines = io::BufReader::new(file).lines();

            for line in lines {
                if let Ok(ip) = line {
                    let row = ip.split('|').collect::<Vec<&str>>();

                    let new_committee = committees::ActiveModel {
                        id: ActiveValue::Set(row[0].to_string()),
                        name: ActiveValue::Set(row[1].to_string()),
                        designation: ActiveValue::Set(row[8].to_string()),
                        org_type: ActiveValue::Set(row[12].to_string()),
                        connected_org: ActiveValue::Set(none_if_empty(row[13])),
                        candidate_id: ActiveValue::Set(none_if_empty(row[14])),
                    };

                    match committees
                        .iter()
                        .position(|x| x.id.as_ref().eq(new_committee.id.as_ref()))
                    {
                        Some(position) => committees[position] = new_committee,
                        None => {
                            committees.insert(committees.len(), new_committee);
                        }
                    };
                }
            }
        }
        return committees;
    }
    pub fn parse_bulk_committee_to_candidate_data() -> Vec<independent_expenditures::ActiveModel> {
        // Data schema: https://www.fec.gov/campaign-finance-data/contributions-committees-candidates-file-description/

        let paths = get_sorted_path_bufs("contributions_from_committee_ind_expenditures/");

        let mut data: Vec<independent_expenditures::ActiveModel> = Vec::new();

        // each path contains bulk data for a single election cycle (2 years)
        // which is why I worry about differing election cycles for each file when I merge rows
        for path in paths {
            let file = File::open(path).expect("error reading file");
            let lines = io::BufReader::new(file).lines();

            let mut merged_rows: Vec<independent_expenditures::ActiveModel> = Vec::new();

            for line in lines {
                if let Ok(ip) = line {
                    let row = ip.split('|').collect::<Vec<&str>>();

                    let support_or_oppose = match row[5] {
                        "24A" => "O",
                        "24E" => "S",
                        _ => continue,
                    }
                    .to_owned();
                    let committee_id = row[0].to_owned();
                    let candidate_fec_id = row[16].to_owned();
                    let election_cycle = match row[4].len() {
                        18 => {
                            let year = row[4][0..4].parse::<i32>().unwrap();
                            (year + year % 2).to_string()
                        }
                        11 => {
                            let year = format!("20{}", &row[4][0..2])
                                .to_string()
                                .parse::<i32>()
                                .unwrap();
                            (year + year % 2).to_string()
                        }
                        _ => panic!("INVALID DATE IN BULK DATA"),
                    };
                    let expenditure_amt = row[14].parse::<i32>().unwrap();

                    match data.iter().position(|x| {
                        x.candidate_id
                            .eq(&ActiveValue::Set(candidate_fec_id.clone()))
                            && x.committee_id.eq(&ActiveValue::set(committee_id.clone()))
                            && x.support_oppose
                                .eq(&ActiveValue::Set(support_or_oppose.clone()))
                    }) {
                        Some(index) => {
                            let item = data.get_mut(index).unwrap();
                            let new_amt = item.amount.as_ref() + expenditure_amt;
                            item.amount = ActiveValue::Set(new_amt);
                        }
                        None => {
                            let item = independent_expenditures::ActiveModel {
                                id: ActiveValue::NotSet,
                                committee_id: ActiveValue::Set(committee_id),
                                candidate_id: ActiveValue::Set(candidate_fec_id),
                                support_oppose: ActiveValue::Set(support_or_oppose),
                                election_cycle: ActiveValue::Set(election_cycle),
                                amount: ActiveValue::Set(expenditure_amt),
                            };
                            data.insert(data.len(), item)
                        }
                    };
                }
            }
            data.append(&mut merged_rows);
        }

        return data;
    }
}
