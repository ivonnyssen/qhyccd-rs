Feature: Constructing an Sdk instance
    
  Scenario: No cameras or filter wheels
    Given I have no cameras
    When I construct and Sdk instance
    Then the Sdk instance should have no cameras
    And the Sdk instance should have no filter wheels

  Scenario: One camera without filter wheel
    Given I have one camera without a filter wheel
    When I construct an Sdk instance
    Then the Sdk instance should have one camera
    And the Sdk instance should have no filter wheels

  Scenario: One camera or with filter wheel
    Given I have one camera with a filter wheel with 5 positions
    When I construct and Sdk instance
    Then the Sdk instance should have one camera
    And the Sdk instance should have one filter wheel with 5 positions