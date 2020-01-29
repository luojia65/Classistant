create table DAuthHash
(
    user_id       int auto_increment
        primary key,
    hash          blob                               null,
    date_register datetime default CURRENT_TIMESTAMP not null
);

create table DAuthId
(
    user_id   int            not null,
    auth_type varchar(32)    not null,
    auth_id   varbinary(256) not null,
    primary key (auth_type, user_id)
);

create index index_auth_id
    on DAuthId (auth_id);

create table DDataUser
(
    user_id      int                                not null,
    type_id      binary(16)                         not null,
    data         blob                               not null,
    encryption   blob                               not null,
    date_modify  datetime default CURRENT_TIMESTAMP not null,
    date_created datetime default CURRENT_TIMESTAMP not null,
    date_expired datetime                           null,
    primary key (user_id, type_id)
);

create table DFormType
(
    form_id int auto_increment
        primary key,
    content text        not null,
    class   varchar(16) not null,
    extra   blob        null
);

create table DGroupMember
(
    group_id      int auto_increment,
    user_id       int                                not null,
    priv          int                                not null,
    date_modified datetime default CURRENT_TIMESTAMP not null,
    date_created  datetime default CURRENT_TIMESTAMP not null,
    date_expired  datetime                           null,
    primary key (group_id, user_id)
);

create
    definer = classistant@localhost procedure PFormTypeCreate(IN _content int, IN _class text, IN _extra blob)
BEGIN
    INSERT INTO `DFormType` (`content`, `class`, `extra`)
    VALUES (`_content`, `_class`, `_extra`);
    SELECT last_insert_id() as `form_id`;
END;

create
    definer = classistant@localhost procedure PGroupCreate(IN _creator_user_id int)
BEGIN
    INSERT INTO `DGroupMember` (`user_id`, `priv`)
    VALUES (`_creator_user_id`, 2);
    SELECT last_insert_id() as `group_id`;
END;

create
    definer = classistant@localhost procedure PGroupDelete(IN _group_id int, IN _operator_user_id int)
BEGIN
    DECLARE `op_priv` INT DEFAULT NULL;
    SELECT `priv`
    INTO `op_priv`
    FROM `DGroupMember`
    WHERE `group_id` = `_group_id`
      AND `user_id` = `_operator_user_id`
      AND (`date_expired` IS NULL OR `date_expired` > CURRENT_TIMESTAMP());
    IF `op_priv` IS NULL THEN
        SELECT 2 as `return_id`; -- group or user not found
    ELSEIF `op_priv` != 2 THEN
        SELECT 1 as `return_id`; -- permission denied
    ELSE
        -- expire all group members
        UPDATE `DGroupMember`
        SET `date_expired` = CURRENT_TIMESTAMP()
        WHERE `group_id` = `_group_id`
          AND `date_expired` IS NULL;
        SELECT 0 as `return_id`; -- success
    END IF;
END;

create
    definer = classistant@localhost procedure PGroupTransferOwner(IN _group_id int, IN _user_id_src int, IN _user_id_dst int)
BEGIN
    DECLARE `op_priv` INT DEFAULT NULL;
    SELECT `priv`
    INTO `op_priv`
    FROM `DGroupMember`
    WHERE `group_id` = `_group_id`
      AND `user_id` = `_user_id_src`;
    IF `op_priv` IS NULL THEN
        SELECT 2 AS `return_id`; -- src user not found
    ELSEIF `op_priv` != 2 THEN
        SELECT 1 AS `return_id`; -- permission denied
    ELSEIF (SELECT `user_id`
            FROM `DGroupMember`
            WHERE `group_id` = `_group_id`
              AND `user_id` = `_user_id_dst`) IS NULL THEN
        SELECT 3 AS `return_id`; -- dest user not found
    ELSE
        START TRANSACTION;
        INSERT INTO `DGroupMember` (`group_id`, `user_id`, `priv`)
        VALUES (`_group_id`, `_user_id_src`, 0)
        ON DUPLICATE KEY UPDATE `priv`          = 0,
                                `date_expired`  = NULL,
                                `date_modified` = CURRENT_TIMESTAMP();
        INSERT INTO `DGroupMember` (`group_id`, `user_id`, `priv`)
        VALUES (`_group_id`, `_user_id_dst`, 2)
        ON DUPLICATE KEY UPDATE `priv`          = 2,
                                `date_expired`  = NULL,
                                `date_modified` = CURRENT_TIMESTAMP();
        COMMIT;
        SELECT 0 AS `return_id`; -- success
    END IF;
END;

create
    definer = classistant@localhost procedure PUserDataDelete(IN _user_id int, IN _type_id binary(16))
BEGIN
    UPDATE `DDataUser`
    SET `date_expired` = CURRENT_TIMESTAMP()
    WHERE `user_id` = `_user_id`
      AND `type_id` = `_type_id`
      AND `date_expired` IS NULL;
END;

create
    definer = classistant@localhost procedure PUserDataGet(IN _user_id int, IN _type_id binary(16))
BEGIN
    SELECT `data`, `encryption`
    FROM `DDataUser`
    WHERE `user_id` = `_user_id`
      AND `type_id` = `_type_id`
      AND `date_expired` IS NULL;
END;

create
    definer = classistant@localhost procedure PUserDataInsert(IN _user_id int, IN _type_id binary(16), IN _data blob,
                                                              IN _encryption blob)
BEGIN
    INSERT INTO `DDataUser` (`user_id`, `type_id`, `data`, `encryption`)
    VALUES (`_user_id`, `_type_id`, `_data`, `_encryption`)
    ON DUPLICATE KEY UPDATE `data` = `_data`, `encryption` = `_encryption`;
END;

create
    definer = classistant@localhost procedure PUserLoginByAuthId(IN _auth_id varbinary(256), IN _hash blob)
BEGIN
    DECLARE `_tmp_user_id` INT;
    DECLARE `_tmp_hash` BLOB;

    SELECT `user_id`
    INTO `_tmp_user_id`
    FROM `DAuthId`
    WHERE `auth_id` = `_auth_id`
    LIMIT 1;

    IF `_tmp_user_id` IS NULL THEN
        SELECT 3 as `return_id`; -- user of this nickname not found
    ELSE
        SELECT `hash`
        INTO `_tmp_hash`
        FROM `DAuthHash`
        WHERE `user_id` = `_tmp_user_id`;
        IF `_tmp_hash` != `_hash` THEN
            SELECT 1 as `return_id`; -- wrong password
        ELSE
            SELECT 0 as `return_id`, `_tmp_user_id` as `user_id`;
        END IF;
    END IF;
END;

create
    definer = classistant@localhost procedure PUserRegisterByNickname(IN _nickname varbinary(256))
BEGIN
    DECLARE `_tmp_user_id` INT;

    SELECT `user_id`
    INTO `_tmp_user_id`
    FROM `DAuthId`
    WHERE `auth_type` = 'nickname'
      AND `auth_id` = `_nickname`;

    IF `_tmp_user_id` IS NOT NULL THEN
        SELECT 1 as `return_id`; -- nickname already taken
    ELSE
        INSERT INTO `DAuthHash` (`hash`) VALUES (NULL);
        SELECT last_insert_id() INTO `_tmp_user_id`;
        INSERT INTO `DAuthId` (`user_id`, `auth_type`, `auth_id`)
        VALUES (`_tmp_user_id`, 'nickname', `_nickname`);
        SELECT 0 as `return_id`, `_tmp_user_id` as `user_id`; -- success
    END IF;
END;

create
    definer = classistant@localhost procedure PUserRegisterFillHash(IN _user_id int, IN _hash blob)
BEGIN
    UPDATE `DAuthHash`
    SET `hash` = `_hash`
    WHERE `user_id` = `_user_id`;
END;

