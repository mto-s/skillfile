use skillfile_functional_tests::sf;

#[test]
fn install_path_deploys_skill_nested_and_agent_flat() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    std::fs::create_dir_all(root.join("skills")).unwrap();
    std::fs::create_dir_all(root.join("agents")).unwrap();
    std::fs::write(root.join("skills/demo-skill.md"), "# Demo Skill\n").unwrap();
    std::fs::write(root.join("agents/demo-agent.md"), "# Demo Agent\n").unwrap();

    std::fs::write(
        root.join("Skillfile"),
        "install-path  custom-skill  skill  ./out/skills\n\
         install-path  custom-agent  agent  ./out/agents\n\
         local  skill  demo-skill  skills/demo-skill.md\n\
         local  agent  demo-agent  agents/demo-agent.md\n",
    )
    .unwrap();

    sf(root).arg("install").assert().success();

    let deployed_skill = root.join("out/skills/demo-skill/SKILL.md");
    assert_eq!(
        std::fs::read_to_string(&deployed_skill).unwrap(),
        "# Demo Skill\n"
    );

    let deployed_agent = root.join("out/agents/demo-agent.md");
    assert_eq!(
        std::fs::read_to_string(&deployed_agent).unwrap(),
        "# Demo Agent\n"
    );

    assert!(!root.join("out/skills/demo-agent/SKILL.md").exists());
    assert!(!root.join("out/agents/demo-skill.md").exists());
}

#[test]
fn install_path_expands_home() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let home = tempfile::tempdir().unwrap();

    std::fs::create_dir_all(root.join("skills")).unwrap();
    std::fs::write(root.join("skills/home-skill.md"), "# Home Skill\n").unwrap();

    std::fs::write(
        root.join("Skillfile"),
        "install-path  home-target  skill  ~/custom-skills\n\
         local  skill  home-skill  skills/home-skill.md\n",
    )
    .unwrap();

    sf(root)
        .env("HOME", home.path())
        .arg("install")
        .assert()
        .success();

    let deployed_skill = home.path().join("custom-skills/home-skill/SKILL.md");
    assert_eq!(
        std::fs::read_to_string(&deployed_skill).unwrap(),
        "# Home Skill\n"
    );
}
