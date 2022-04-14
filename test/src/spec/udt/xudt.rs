use tempfile::tempdir;

use super::{check_amount, create_acp_cell, prepare, ACCOUNT1_ADDR, ACCOUNT2_ADDR, OWNER_ADDR};
use crate::setup::Setup;
use crate::spec::Spec;

pub struct XudtIssueToAcp;

impl Spec for XudtIssueToAcp {
    fn run(&self, setup: &mut Setup) {
        let tempdir = tempdir().expect("create tempdir failed");
        let path = tempdir.path().to_str().unwrap();
        let owner_key_path = format!("{}/owner", path);
        let account1_key_path = format!("{}/account1", path);
        let account2_key_path = format!("{}/account2", path);
        let cell_deps_path = format!("{}/cell_deps.json", path);
        let xudt_rce_args = prepare(setup, path);

        let account1_acp_addr = create_acp_cell(
            setup,
            OWNER_ADDR,
            Some(xudt_rce_args.as_str()),
            cell_deps_path.as_str(),
            ACCOUNT1_ADDR,
            account1_key_path.as_str(),
        );
        check_amount(
            setup,
            OWNER_ADDR,
            Some(xudt_rce_args.as_str()),
            cell_deps_path.as_str(),
            account1_acp_addr.as_str(),
            0,
        );
        let output = setup.cli(&format!(
            "udt issue --owner {} --xudt-rce-args {} --udt-to {}:300 --to-acp-address --cell-deps {} --privkey-path {}",
            OWNER_ADDR,
            xudt_rce_args,
            account1_acp_addr,
            cell_deps_path,
            owner_key_path,
        ));
        log::info!(
            "Issue 300 XUDT to account 1's anyone-can-pay address:\n{}",
            output
        );
        setup.miner().generate_blocks(3);
        check_amount(
            setup,
            OWNER_ADDR,
            Some(xudt_rce_args.as_str()),
            cell_deps_path.as_str(),
            account1_acp_addr.as_str(),
            300,
        );

        // issue to multiple acp addresses
        let account2_acp_addr = create_acp_cell(
            setup,
            OWNER_ADDR,
            Some(xudt_rce_args.as_str()),
            cell_deps_path.as_str(),
            ACCOUNT2_ADDR,
            account2_key_path.as_str(),
        );
        let output = setup.cli(&format!(
            "udt issue --owner {} --xudt-rce-args {} --udt-to {}:200 --udt-to {}:400 --to-acp-address --cell-deps {} --privkey-path {}",
            OWNER_ADDR,
            xudt_rce_args,
            account1_acp_addr,
            account2_acp_addr,
            cell_deps_path,
            owner_key_path,
        ));
        log::info!(
            "Issue 200 XUDT to account 1 and 400 to account 2:\n{}",
            output
        );
        setup.miner().generate_blocks(3);

        check_amount(
            setup,
            OWNER_ADDR,
            Some(xudt_rce_args.as_str()),
            cell_deps_path.as_str(),
            account1_acp_addr.as_str(),
            500,
        );
        check_amount(
            setup,
            OWNER_ADDR,
            Some(xudt_rce_args.as_str()),
            cell_deps_path.as_str(),
            account2_acp_addr.as_str(),
            400,
        );
    }
}

pub struct XudtTransferToMultiAcp;

impl Spec for XudtTransferToMultiAcp {
    fn run(&self, setup: &mut Setup) {
        let tempdir = tempdir().expect("create tempdir failed");
        let path = tempdir.path().to_str().unwrap();
        let owner_key_path = format!("{}/owner", path);
        let account1_key_path = format!("{}/account1", path);
        let account2_key_path = format!("{}/account2", path);
        let cell_deps_path = format!("{}/cell_deps.json", path);
        let xudt_rce_args = prepare(setup, path);

        let owner_acp_addr = create_acp_cell(
            setup,
            OWNER_ADDR,
            Some(xudt_rce_args.as_str()),
            cell_deps_path.as_str(),
            OWNER_ADDR,
            owner_key_path.as_str(),
        );
        let account1_acp_addr = create_acp_cell(
            setup,
            OWNER_ADDR,
            Some(xudt_rce_args.as_str()),
            cell_deps_path.as_str(),
            ACCOUNT1_ADDR,
            account1_key_path.as_str(),
        );
        let account2_acp_addr = create_acp_cell(
            setup,
            OWNER_ADDR,
            Some(xudt_rce_args.as_str()),
            cell_deps_path.as_str(),
            ACCOUNT2_ADDR,
            account2_key_path.as_str(),
        );
        let output = setup.cli(&format!(
            "udt issue --owner {} --xudt-rce-args {} --udt-to {}:300 --to-acp-address --cell-deps {} --privkey-path {}",
            OWNER_ADDR,
            xudt_rce_args,
            account1_acp_addr,
            cell_deps_path,
            owner_key_path,
        ));
        log::info!("Issue 300 XUDT to account 1's acp address:\n{}", output);
        setup.miner().generate_blocks(3);
        let output = setup.cli(&format!(
            "udt transfer --owner {} --xudt-rce-args {} --sender {} --udt-to {}:150 --udt-to {}:100 --to-acp-address --cell-deps {} --privkey-path {}",
            OWNER_ADDR,
            xudt_rce_args,
            account1_acp_addr,
            owner_acp_addr,
            account2_acp_addr,
            cell_deps_path,
            account1_key_path,
        ));
        log::info!(
            "Transfer 150 XUDT to owner, 100 XUDT to account 2, from account 1:\n{}",
            output
        );
        setup.miner().generate_blocks(3);
        check_amount(
            setup,
            OWNER_ADDR,
            Some(xudt_rce_args.as_str()),
            cell_deps_path.as_str(),
            owner_acp_addr.as_str(),
            150,
        );
        check_amount(
            setup,
            OWNER_ADDR,
            Some(xudt_rce_args.as_str()),
            cell_deps_path.as_str(),
            account1_acp_addr.as_str(),
            50,
        );
        check_amount(
            setup,
            OWNER_ADDR,
            Some(xudt_rce_args.as_str()),
            cell_deps_path.as_str(),
            account2_acp_addr.as_str(),
            100,
        );
    }
}
