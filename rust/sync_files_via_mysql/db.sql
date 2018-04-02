create table users (
    id int(32) unsigned auto_increment primary key,
    name varchar(64) not null,
    `host` varchar(64) not null
);
create table files (
    id int(32) unsigned auto_increment primary key,
    folder_id int(32) unsigned not null,

    foreign key(folder_id) references folders(id)
);
create table folders (
    id int(32) unsigned auto_increment primary key,
    name varchar(64) not null,
    parent_folder_id int(32) unsigned null,

    foreign key(parent_folder_id) references folders(id)
);
create table blobs (
    id int(32) unsigned auto_increment primary key,
    `blob` longblob not null,
    mtime timestamp not null,
    file_id int(32) unsigned not null,
    user_id int(32) unsigned not null,

    foreign key(file_id) references files(id),
    foreign key(user_id) references users(id)
);
create table downloads (
    blob_id int(32) unsigned not null,
    user_id int(32) unsigned not null,
    `time` timestamp not null,

    foreign key(blob_id) references blobs(id),
    foreign key(user_id) references users(id)
);
create table tags (
    id int(32) unsigned auto_increment primary key,
    tag varchar(64) not null
);
create table file_tags (
    file_id int(32) unsigned not null,
    tag_id int(32) unsigned not null,

    foreign key(file_id) references files(id),
    foreign key(tag_id) references tags(id)
);
create table folder_tags (
    folder_id int(32) unsigned not null,
    tag_id int(32) unsigned not null,

    foreign key(folder_id) references folders(id),
    foreign key(tag_id) references tags(id)
);

