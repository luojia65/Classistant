CREATE TABLE IF NOT EXISTS `DUserAuth` (
  `user_id` int(11) NOT NULL AUTO_INCREMENT,
  `nickname` varchar(48) NOT NULL,
  `hash` blob NOT NULL,
  `date_register` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`user_id`)
);

DROP PROCEDURE IF EXISTS `PUserRegister`;

CREATE PROCEDURE `PUserRegister`(
	IN `_nickname` VARCHAR(48),
	IN `_hash` BLOB
)
BEGIN
	DECLARE `_tmp_user_id` INT;
    
	SELECT `user_id` 
  INTO `_tmp_user_id`
  FROM `DUserAuth` WHERE `nickname` = `_nickname`;
    
  IF `_tmp_user_id` IS NOT NULL THEN 
    SELECT 1 as `return_id`; -- nickname already taken
	ELSE
    INSERT INTO `DUserAuth` (`nickname`,`hash`) 
    VALUES (`_nickname`,`_hash`);
    SELECT `user_id` INTO `_tmp_user_id` FROM `DUserAuth` WHERE `nickname` = `_nickname`;
    SELECT 0 as `return_id`, `_tmp_user_id` as `user_id`; -- success
  END IF;
END

DROP PROCEDURE IF EXISTS `PUserLoginById`;

CREATE PROCEDURE `PUserLoginById`(
	IN `_user_id` INT,
	IN `_hash` BLOB
)
BEGIN
	DECLARE `_tmp_nickname` VARCHAR(48);
    DECLARE `_tmp_hash` BLOB;
    
	SELECT `nickname`, `hash`
    INTO `_tmp_nickname`, `_tmp_hash`
    FROM `DUserAuth` WHERE `user_id` = `_user_id`;
	
    IF `_tmp_nickname` IS NULL THEN 
		SELECT 2 as `return_id`; -- user of this id not found
	ELSE 
		IF `_tmp_hash` != `_hash` THEN
			SELECT 1 as `return_id`; -- wrong password
		ELSE 
			SELECT 0 as `return_id`, `_user_id` as `user_id`, `_tmp_nickname` as `nickname`;
		END IF;
	END IF;
END

DROP PROCEDURE IF EXISTS `PUserLoginByNickname`;

CREATE PROCEDURE `PUserLoginByNickname`(
	IN `_nickname` VARCHAR(48),
	IN `_hash` BLOB
)
BEGIN
	DECLARE `_tmp_user_id` INT;
    DECLARE `_tmp_hash` BLOB;
    
	SELECT `user_id`, `hash`
    INTO `_tmp_user_id`, `_tmp_hash`
    FROM `DUserAuth` WHERE `nickname` = `_nickname`;
	
    IF `_tmp_user_id` IS NULL THEN 
		SELECT 3 as `return_id`; -- user of this nickname not found
	ELSE 
		IF `_tmp_hash` != `_hash` THEN
			SELECT 1 as `return_id`; -- wrong password
		ELSE 
			SELECT 0 as `return_id`, `_tmp_user_id` as `user_id`, `_nickname` as `nickname`;
		END IF;
	END IF;
END

CREATE TABLE IF NOT EXISTS `DDataUser` (
  `user_id` int(11) NOT NULL,
  `group_id` int(11) NOT NULL,
  `type_id` binary(16) NOT NULL,
  `data` blob NOT NULL,
  `date_modify` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`user_id`,`group_id`,`type_id`)
);

DROP PROCEDURE IF EXISTS `PDataUInsert`;

CREATE PROCEDURE `PDataUInsert`(
	IN `_user_id` INT,
    IN `_group_id` INT,
    IN `_type_id` BINARY(16),
	IN `_data` BLOB
)
BEGIN
    INSERT INTO `DDataUser` (`user_id`,`group_id`,`type_id`,`data`)
	VALUES (`_user_id`, `_group_id`,`_type_id`,`_data`)
    ON DUPLICATE KEY UPDATE `data` = `_data`;
END

DROP PROCEDURE IF EXISTS `PDataUGet`;

CREATE PROCEDURE `PDataUGet`(
	IN `_user_id` INT,
    IN `_group_id` INT,
    IN `_type_id` BINARY(16)
)
BEGIN
    SELECT `data` FROM `DDataUser` 
    WHERE `user_id` = `_user_id` AND
		  `group_id` = `_group_id` AND
          `type_id` = `_type_id`;
END
