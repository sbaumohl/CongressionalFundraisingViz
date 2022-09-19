pub mod fec_data_handler {
    use std::{
        fs::{self, File},
        io::{self, BufRead},
        path::{Path},
    };

    const DATA_PATH: &str = "./src/bin/";
    // Data from: https://www.fec.gov/data/browse-data/?tab=bulk-data

    #[derive(Debug)]
    pub struct CommitteeToCandidateData {
        committee: String,
        expenditure_amount: i32,
        support_or_oppose: OpposeSupportIndicator,
        election_cycle: String,
        candidate_fec_id: String,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum OpposeSupportIndicator {
        Support,
        Oppose,
    }

    pub fn parse_bulk_committees() -> Vec<Committee> {
        todo!();
         /*
         * Data schema: https://www.fec.gov/campaign-finance-data/committee-master-file-description/
         */
        let root_path = Path::new(DATA_PATH).join("committees/");
        let paths = fs::read_dir(root_path).unwrap();

        let mut data: Vec<CommitteeToCandidateData> = Vec::new();

        // each path contains bulk data for a single election cycle (2 years)
        // which is why I worry about differing election cycles for each file when I merge rows

        for path in paths {
            let absolute_dir = path.expect("error decoding director").path();
            let file = File::open(absolute_dir).expect("error reading file");
            let lines = io::BufReader::new(file).lines();

            let mut merged_rows: Vec<CommitteeToCandidateData> = Vec::new();

            for line in lines {
                if let Ok(ip) = line {
                    let row = ip.split('|').collect::<Vec<&str>>();

                    let support_or_oppose = match row[5] {
                        "24A" => OpposeSupportIndicator::Oppose,
                        "24E" => OpposeSupportIndicator::Support,
                        _ => continue,
                    };
                    let committee = row[0].to_owned();
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
                        x.candidate_fec_id == candidate_fec_id
                            && x.support_or_oppose == support_or_oppose
                            && x.committee == committee
                    }) {
                        Some(index) => data.get_mut(index).unwrap().expenditure_amount += expenditure_amt,
                        None => data.insert(
                            data.len(),
                            CommitteeToCandidateData {
                                committee,
                                expenditure_amount: expenditure_amt,
                                support_or_oppose,
                                election_cycle,
                                candidate_fec_id,
                            },
                        ),
                    };
                }
            }
            data.append(&mut merged_rows);
        }

        return data;
    }

    pub fn parse_bulk_committee_to_candidate_data() -> Vec<CommitteeToCandidateData> {
        /*
         * Data schema: https://www.fec.gov/campaign-finance-data/contributions-committees-candidates-file-description/
         */
        let root_path = Path::new(DATA_PATH).join("contributions_from_committee_ind_expenditures/");
        let paths = fs::read_dir(root_path).unwrap();

        let mut data: Vec<CommitteeToCandidateData> = Vec::new();

        // each path contains bulk data for a single election cycle (2 years)
        // which is why I worry about differing election cycles for each file when I merge rows

        for path in paths {
            let absolute_dir = path.expect("error decoding director").path();
            let file = File::open(absolute_dir).expect("error reading file");
            let lines = io::BufReader::new(file).lines();

            let mut merged_rows: Vec<CommitteeToCandidateData> = Vec::new();

            for line in lines {
                if let Ok(ip) = line {
                    let row = ip.split('|').collect::<Vec<&str>>();

                    let support_or_oppose = match row[5] {
                        "24A" => OpposeSupportIndicator::Oppose,
                        "24E" => OpposeSupportIndicator::Support,
                        _ => continue,
                    };
                    let committee = row[0].to_owned();
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
                        x.candidate_fec_id == candidate_fec_id
                            && x.support_or_oppose == support_or_oppose
                            && x.committee == committee
                    }) {
                        Some(index) => data.get_mut(index).unwrap().expenditure_amount += expenditure_amt,
                        None => data.insert(
                            data.len(),
                            CommitteeToCandidateData {
                                committee,
                                expenditure_amount: expenditure_amt,
                                support_or_oppose,
                                election_cycle,
                                candidate_fec_id,
                            },
                        ),
                    };
                }
            }
            data.append(&mut merged_rows);
        }

        return data;
    }
}