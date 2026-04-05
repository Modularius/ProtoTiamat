# Proto Tiamat

## Crates in Project

 - Libertee: library containing server and user types.
 - Communitee: leptos implementation of the site, implementing `libertee` library.
 - Facilitee: leptos implementation of the site, implementing `libertee` library.
 - Abilitee: leptos implementation of the site, implementing `libertee` library.
 - Virtualitee: simulation of a server from the `libertee` library, and of several users.

## Structures in Libertee

- User: Represents a user of communitee, each user has a board onto which only they can post. This user-board is visible to themselves, and their friends.
- Board: A collection of posts written by one or more user.
- Post: A message written by a user, hosted on a board, posts can contain a nested hierarchy of posts as replies.
- Group: A group hosts a board and has a collection of users, known as members. All members can view the group's board, and can post to it. The group also maintains a weight associated to each post, which defines how likely it is to appear to the group's members.
- Message: a private message written by a user, to another user.
- Credentials: a username/password pair which is assoicated to a user.
- Member Weight: a value assigned to each member of a group. This defines how much their promotions count towards a group post's weight. A member's weight is originally `1`, but they can assign a portion that weight to another member. This member, can in turn, assign a portion of their weight to another member, and so on, as long as a cycle is not created.
- Member: users subscribed to a group are called members. Each member maintains a list of delegates, that is other group members to which the member assign a portion of their own group weight.
- Delegate: a group member to which another member assigns a portion of their weight. The member assigning their weight is said to be a delegator of their delegate.

### User Structure
|Property|Type|Description|
|---|---|---|
|Id|UserUuid|The user's own unique id.|
|Credentials|username/password pair|This is used by a client to login as this user|
|Name|String|The personal name of this user. This is deliberately unstructured, as this is only meant to be human readable|
|Properties|String -> String|A dictionary of the user's attributes. At the moment, there are no plans for this to be further structures as this should only be for human comprehension.|
|Groups|GroupUuid -> MemberUuid|A dictionary assigning a member id, to each group the user is a member of.|
|Friends|UserUuid->Friendship|A dictionary assigning a friendship object to every user the user is friends with.|
|Banned|UserUuid->Ban|A dictionary assigning a ban object to every user the user has banned.|
|Posts|Board|The user's personal post board.|
|History|[Action]|An immutable sequential list of all actions the user has done.|
|BoardsViewed|

### Group Structure
|Property|Type|Description|
|---|---|---|
|Id|GroupUuid|The group's own unique id.|
|Posts|Board|The group's postboard.|
|Weight|PostUuid -> Weight|A dictionary assigning each post a weight object.|
|PostWeights|Real -> {PostUuid}|A BTree(Multi)Map which assigns a set of posts, which share a weight, to their shared weight, and stores them in descening order. This allows a feed to be generated with the most popular posts, more likely to be shown.
|Members|MemberUuid -> Member|Dictionary assigning a member object to each member id.|
|Admins|MemberUuid -> Admin||

