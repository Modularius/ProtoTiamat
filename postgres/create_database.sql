CREATE TYPE UserId as INT

CREATE TABLE Users (
    user_id UserId IDENTITY,
    user_name VARCHAR,
    joined_on DATETIME,
)

CREATE TABLE UserProperties (
    user_id UserId,
    property_name CHAR (128),
    property_value VARCHAR,
)

CREATE TABLE Friendship (
    user1 UserId,
    user2 UserId,
    FRIENDSHIP_ON DATETIME,
    CHECK validate_users user1 < user2
)

CREATE TABLE Block (
    blocking_user UserId,
    blocked_user UserId,
    blocked_on DATETIME,
    reason VARCHAR,
    CHECK validate_users blocking_user is not blocked_user
)

CREATE TYPE GroupId as INT

CREATE TABLE Group (
    group_id GroupId IDENTITY,
    group_name CHAR (128),
    group_description CHAR (4096),
)

CREATE TYPE MemberId as INT

CREATE TABLE GroupMember (
    member_id MemberId IDENTITY,
    group_id GroupId,
    user_id UserId,
    joined_on DATETIME,
)

CREATE TABLE Delegation (
    group_id GroupId,
    delegator MemberId,
    delegee MemberId,
    wgt Float,
    CHECK validate_delegation delegator is not delegee
)

CREATE TYPE GroupRole AS ENUM ("SuperAdmin", "Admin", "Moderator")

CREATE TABLE GroupAdmin (
    group_id GroupId,
    member_id MemberId,
    group_role GroupRole,
    instated_on DATETIME,
    power Float,
)

CREATE TABLE GroupCandidate (
    group_id GroupId,
    member_id MemberId,
    desired_role GroupRole,
    candidacy_on DATETIME,
)

CREATE TYPE PostTypeDef as ENUM ('Personal Post', 'Group Post', 'Public Reply', 'PrivateReply', 'Private Message')

CREATE TYPE PostOrGroupId as INT
CREATE TYPE UserOrMemberId as INT
CREATE TYPE PostType as (
    author_id UserOrMemberId,
    def PostTypeDef,
    link_id PostOrGroupId,
)

CREATE TYPE PostId as INT

CREATE TABLE Post (
    post_id PostId IDENTITY,
    post_type PostType,
    title CHAR(256),
    body VARCHAR,
    posted_on DATETIME,
    CHECK validate_PostType ((post_type.link_id IS NOT NULL) OR (post_type.def = 'Personal Post'))
)