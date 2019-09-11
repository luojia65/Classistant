CREATE TABLE IF NOT EXISTS `DUserAuth` (
  `user_id` int(11) NOT NULL AUTO_INCREMENT,
  `nickname` varchar(48) NOT NULL,
  `hash` blob NOT NULL,
  `date_register` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`user_id`)
);

DROP PROCEDURE `PUserRegister`;

CREATE PROCEDURE `PUserRegister`(
	IN `_nickname` VARCHAR(48),
	IN `_hash` BLOB
)
BEGIN
	DECLARE `_tmp_user_id` VARCHAR(48);
    
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

DROP PROCEDURE `PUserLoginById`;

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
			SELECT 0 as `return_id`, `_tmp_nickname` as `nickname`;
		END IF;
	END IF;
END

DROP PROCEDURE `PUserLoginByNickname`;

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
		SELECT 2 as `return_id`; -- user of this nickname not found
	ELSE 
		IF `_tmp_hash` != `_hash` THEN
			SELECT 1 as `return_id`; -- wrong password
		ELSE 
			SELECT 0 as `return_id`, `_tmp_user_id` as `user_id`;
		END IF;
	END IF;
END
