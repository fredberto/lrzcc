use std::str::FromStr;

use avina::{Api, Token};
use avina_test::{random_alphanumeric_string, spawn_app};

#[tokio::test]
async fn e2e_lib_flavor_group_modify_denies_access_to_normal_user() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let user = test_project.normals[0].user.clone();
    let token = test_project.normals[0].token.clone();
    let project = test_project.project.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor_group = server
        .setup_test_flavor_group(project.id)
        .await
        .expect("Failed to setup test server state");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let modify = client.flavor_group.modify(flavor_group.id).send().await;

    // assert
    assert!(modify.is_err());
    assert_eq!(
        modify.unwrap_err().to_string(),
        format!("Admin privileges required")
    );
}

#[tokio::test]
async fn e2e_lib_flavor_group_modify_denies_access_to_master_user() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();
    let project = test_project.project.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor_group = server
        .setup_test_flavor_group(project.id)
        .await
        .expect("Failed to setup test server state");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let modify = client.flavor_group.modify(flavor_group.id).send().await;

    // assert
    assert!(modify.is_err());
    assert_eq!(
        modify.unwrap_err().to_string(),
        format!("Admin privileges required")
    );
}

#[tokio::test]
async fn e2e_lib_flavor_group_modify_and_get_works() {
    // arrange
    let server = spawn_app().await;
    let (user, project, token) = server
        .setup_test_user_and_project(true)
        .await
        .expect("Failed to setup test user and project.");
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor_group = server
        .setup_test_flavor_group(project.id)
        .await
        .expect("Failed to setup test server state");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act and assert 1 - modify
    let name = random_alphanumeric_string(10);
    let modified = client
        .flavor_group
        .modify(flavor_group.id)
        .name(name.clone())
        // TODO: test changing the project
        .send()
        .await
        .unwrap();
    assert_eq!(&name, &modified.name);

    // act and assert 2 - get
    let retrieved = client.flavor_group.get(modified.id).await.unwrap();
    assert_eq!(modified.id, retrieved.id);
    assert_eq!(modified.name, retrieved.name);
    assert_eq!(modified.project, retrieved.project.id);
    assert_eq!(
        modified.flavors,
        retrieved
            .flavors
            .into_iter()
            .map(|f| f.id)
            .collect::<Vec<u32>>()
    );
}
