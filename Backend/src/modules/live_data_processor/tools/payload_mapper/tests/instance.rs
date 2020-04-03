use crate::modules::live_data_processor::tools::payload_mapper::instance::MapInstance;

#[test]
fn map_instance_positive_with_winner() {
  // Arrange
  let payload = vec![
    22, 0, 0, 0, // MapId
    33, 0, 0, 0, // InstanceId
    2
  ];

  // Act
  let result = payload.to_instance();

  // Assert
  assert!(result.is_ok());
  let instance = result.unwrap();
  assert_eq!(instance.map_id, 22);
  assert_eq!(instance.instance_id, 33);
  assert_eq!(instance.winner, Some(2));
}

#[test]
fn map_instance_positive_without_winner() {
  // Arrange
  let payload = vec![
    22, 0, 0, 0, // MapId
    33, 0, 0, 0 // InstanceId
  ];

  // Act
  let result = payload.to_instance();

  // Assert
  assert!(result.is_ok());
  let instance = result.unwrap();
  assert_eq!(instance.map_id, 22);
  assert_eq!(instance.instance_id, 33);
  assert_eq!(instance.winner, None);
}

#[test]
fn map_instance_negative() {
  // Arrange
  let payload = vec![1,2,3,4,5];

  // Act
  let result = payload.to_instance();

  // Assert
  assert!(result.is_err());
}