-- mutsea-database/src/sql/postgresql/schema/create_opensim_compatibility.sql
-- OpenSim Compatibility Tables

-- Regions table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS regions (
    uuid UUID PRIMARY KEY,
    regionHandle BIGINT NOT NULL UNIQUE,
    regionName VARCHAR(32) NOT NULL,
    regionRecvKey VARCHAR(128),
    regionSendKey VARCHAR(128),
    regionSecret VARCHAR(128),
    regionDataURI VARCHAR(255),
    serverIP VARCHAR(64),
    serverPort INTEGER,
    serverURI VARCHAR(255),
    locX INTEGER NOT NULL,
    locY INTEGER NOT NULL,
    locZ INTEGER DEFAULT 0,
    eastOverrideHandle BIGINT,
    westOverrideHandle BIGINT,
    southOverrideHandle BIGINT,
    northOverrideHandle BIGINT,
    regionAssetURI VARCHAR(255),
    regionAssetRecvKey VARCHAR(128),
    regionAssetSendKey VARCHAR(128),
    regionUserURI VARCHAR(255),
    regionUserRecvKey VARCHAR(128),
    regionUserSendKey VARCHAR(128),
    regionMapTexture UUID,
    serverHttpPort INTEGER DEFAULT 9000,
    serverRemotingPort INTEGER DEFAULT 8895,
    access INTEGER DEFAULT 1,
    ScopeID UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    sizeX INTEGER DEFAULT 256,
    sizeY INTEGER DEFAULT 256,
    flags INTEGER DEFAULT 0,
    last_seen INTEGER DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_regions_handle ON regions(regionHandle);
CREATE INDEX IF NOT EXISTS idx_regions_name ON regions(regionName);
CREATE INDEX IF NOT EXISTS idx_regions_location ON regions(locX, locY);

-- User accounts table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS user_accounts (
    PrincipalID UUID PRIMARY KEY,
    ScopeID UUID NOT NULL,
    FirstName VARCHAR(64) NOT NULL,
    LastName VARCHAR(64) NOT NULL,
    Email VARCHAR(64),
    ServiceURLs TEXT,
    Created INTEGER NOT NULL,
    UserLevel INTEGER DEFAULT 0,
    UserFlags INTEGER DEFAULT 0,
    UserTitle VARCHAR(64) DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_user_accounts_name ON user_accounts(FirstName, LastName);
CREATE INDEX IF NOT EXISTS idx_user_accounts_email ON user_accounts(Email);

-- Assets table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS assets (
    id UUID PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    description VARCHAR(64) NOT NULL,
    assetType SMALLINT NOT NULL,
    local BOOLEAN NOT NULL,
    temporary BOOLEAN NOT NULL,
    data BYTEA,
    create_time INTEGER,
    access_time INTEGER,
    asset_flags INTEGER DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_assets_name ON assets(name);
CREATE INDEX IF NOT EXISTS idx_assets_type ON assets(assetType);
CREATE INDEX IF NOT EXISTS idx_assets_local ON assets(local);

-- Inventory folders table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS inventoryfolders (
    folderID UUID PRIMARY KEY,
    agentID UUID NOT NULL,
    parentFolderID UUID NOT NULL,
    folderName VARCHAR(64),
    type SMALLINT NOT NULL DEFAULT 8,
    version INTEGER NOT NULL DEFAULT 1
);

CREATE INDEX IF NOT EXISTS idx_inventoryfolders_agent ON inventoryfolders(agentID);
CREATE INDEX IF NOT EXISTS idx_inventoryfolders_parent ON inventoryfolders(parentFolderID);

-- Inventory items table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS inventoryitems (
    inventoryID UUID PRIMARY KEY,
    assetID UUID,
    assetType INTEGER,
    parentFolderID UUID NOT NULL,
    avatarID UUID NOT NULL,
    inventoryName VARCHAR(64),
    inventoryDescription VARCHAR(128),
    inventoryNextPermissions INTEGER,
    inventoryCurrentPermissions INTEGER,
    invType INTEGER,
    creatorID UUID NOT NULL,
    inventoryBasePermissions INTEGER NOT NULL,
    inventoryEveryOnePermissions INTEGER NOT NULL,
    inventoryGroupPermissions INTEGER NOT NULL,
    salePrice INTEGER DEFAULT 0,
    saleType SMALLINT DEFAULT 0,
    creationDate INTEGER DEFAULT 0,
    groupID UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    groupOwned BOOLEAN DEFAULT FALSE,
    flags INTEGER DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_inventoryitems_avatar ON inventoryitems(avatarID);
CREATE INDEX IF NOT EXISTS idx_inventoryitems_folder ON inventoryitems(parentFolderID);

-- Primitives table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS primitives (
    UUID UUID PRIMARY KEY,
    RegionUUID UUID,
    CreationDate INTEGER,
    Name VARCHAR(255),
    Text VARCHAR(255),
    Description VARCHAR(255),
    SitName VARCHAR(255),
    TouchName VARCHAR(255),
    ObjectFlags INTEGER,
    CreatorID UUID,
    OwnerID UUID,
    GroupID UUID,
    LastOwnerID UUID,
    OwnerMask INTEGER,
    NextOwnerMask INTEGER,
    GroupMask INTEGER,
    EveryoneMask INTEGER,
    BaseMask INTEGER,
    PositionX DOUBLE PRECISION,
    PositionY DOUBLE PRECISION,
    PositionZ DOUBLE PRECISION,
    GroupPositionX DOUBLE PRECISION,
    GroupPositionY DOUBLE PRECISION,
    GroupPositionZ DOUBLE PRECISION,
    VelocityX DOUBLE PRECISION,
    VelocityY DOUBLE PRECISION,
    VelocityZ DOUBLE PRECISION,
    AngularVelocityX DOUBLE PRECISION,
    AngularVelocityY DOUBLE PRECISION,
    AngularVelocityZ DOUBLE PRECISION,
    AccelerationX DOUBLE PRECISION,
    AccelerationY DOUBLE PRECISION,
    AccelerationZ DOUBLE PRECISION,
    RotationX DOUBLE PRECISION,
    RotationY DOUBLE PRECISION,
    RotationZ DOUBLE PRECISION,
    RotationW DOUBLE PRECISION,
    SitTargetOffsetX DOUBLE PRECISION,
    SitTargetOffsetY DOUBLE PRECISION,
    SitTargetOffsetZ DOUBLE PRECISION,
    SitTargetOrientX DOUBLE PRECISION,
    SitTargetOrientY DOUBLE PRECISION,
    SitTargetOrientZ DOUBLE PRECISION,
    SitTargetOrientW DOUBLE PRECISION,
    ScaleX DOUBLE PRECISION,
    ScaleY DOUBLE PRECISION,
    ScaleZ DOUBLE PRECISION,
    PCode INTEGER,
    PathBegin INTEGER,
    PathEnd INTEGER,
    PathScaleX INTEGER,
    PathScaleY INTEGER,
    PathShearX INTEGER,
    PathShearY INTEGER,
    PathSkew INTEGER,
    PathCurve INTEGER,
    PathRadiusOffset INTEGER,
    PathRevolutions INTEGER,
    PathTaperX INTEGER,
    PathTaperY INTEGER,
    PathTwist INTEGER,
    PathTwistBegin INTEGER,
    ProfileBegin INTEGER,
    ProfileEnd INTEGER,
    ProfileCurve INTEGER,
    ProfileHollow INTEGER,
    Texture BYTEA,
    ExtraParams BYTEA,
    State INTEGER
);

CREATE INDEX IF NOT EXISTS idx_primitives_region ON primitives(RegionUUID);
CREATE INDEX IF NOT EXISTS idx_primitives_owner ON primitives(OwnerID);
CREATE INDEX IF NOT EXISTS idx_primitives_creator ON primitives(CreatorID);

-- Terrain table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS terrain (
    RegionUUID UUID NOT NULL,
    Revision INTEGER NOT NULL,
    Heightfield BYTEA,
    PRIMARY KEY (RegionUUID, Revision)
);

-- Land/Parcels table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS land (
    UUID UUID PRIMARY KEY,
    RegionUUID UUID,
    LocalLandID INTEGER,
    Bitmap BYTEA,
    Name VARCHAR(255),
    Description VARCHAR(255),
    OwnerUUID UUID,
    IsGroupOwned BOOLEAN,
    Area INTEGER,
    AuctionID INTEGER,
    Category INTEGER,
    ClaimDate INTEGER,
    ClaimPrice INTEGER,
    GroupUUID UUID,
    SalePrice INTEGER,
    LandStatus INTEGER,
    LandFlags INTEGER,
    LandingType INTEGER,
    MediaAutoScale INTEGER,
    MediaTextureUUID UUID,
    MediaURL VARCHAR(255),
    MusicURL VARCHAR(255),
    PassHours DOUBLE PRECISION,
    PassPrice INTEGER,
    SnapshotUUID UUID,
    UserLocationX DOUBLE PRECISION,
    UserLocationY DOUBLE PRECISION,
    UserLocationZ DOUBLE PRECISION,
    UserLookAtX DOUBLE PRECISION,
    UserLookAtY DOUBLE PRECISION,
    UserLookAtZ DOUBLE PRECISION,
    AuthbuyerID UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    OtherCleanTime INTEGER DEFAULT 0,
    Dwell DOUBLE PRECISION DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_land_region ON land(RegionUUID);
CREATE INDEX IF NOT EXISTS idx_land_owner ON land(OwnerUUID);

-- Land access list table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS landaccesslist (
    LandUUID UUID,
    AccessUUID UUID,
    Flags INTEGER,
    Expires INTEGER DEFAULT 0,
    PRIMARY KEY (LandUUID, AccessUUID)
);

-- Additional OpenSim tables for complete compatibility

-- Presence table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS presence (
    UserID UUID NOT NULL,
    RegionID UUID NOT NULL,
    SessionID UUID NOT NULL,
    SecureSessionID UUID NOT NULL,
    PRIMARY KEY (UserID, RegionID)
);

-- GridUser table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS griduser (
    UserID UUID PRIMARY KEY,
    HomeRegionID UUID,
    HomePosition VARCHAR(64),
    HomeLookAt VARCHAR(64),
    LastRegionID UUID,
    LastPosition VARCHAR(64),
    LastLookAt VARCHAR(64),
    Online BOOLEAN,
    Login INTEGER,
    Logout INTEGER
);

-- Friends table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS friends (
    PrincipalID UUID,
    Friend VARCHAR(255),
    Flags INTEGER NOT NULL DEFAULT 0,
    Offered VARCHAR(32) NOT NULL DEFAULT 0,
    PRIMARY KEY (PrincipalID, Friend)
);

-- Groups table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS os_groups_groups (
    GroupID UUID PRIMARY KEY,
    Location VARCHAR(255) DEFAULT '',
    Name VARCHAR(255) NOT NULL,
    Charter TEXT,
    InsigniaID UUID,
    FounderID UUID,
    MembershipFee INTEGER DEFAULT 0,
    OpenEnrollment VARCHAR(255) DEFAULT 'false',
    ShowInList INTEGER DEFAULT 1,
    AllowPublish INTEGER DEFAULT 1,
    MaturePublish INTEGER DEFAULT 1,
    OwnerRoleID UUID
);

-- Group members table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS os_groups_membership (
    GroupID UUID,
    PrincipalID UUID,
    SelectedRoleID UUID,
    Contribution INTEGER DEFAULT 0,
    ListInProfile INTEGER DEFAULT 1,
    AcceptNotices INTEGER DEFAULT 1,
    AccessToken UUID,
    PRIMARY KEY (GroupID, PrincipalID)
);

-- Group roles table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS os_groups_roles (
    GroupID UUID,
    RoleID UUID,
    Name VARCHAR(255) NOT NULL,
    Description VARCHAR(255),
    Title VARCHAR(255),
    Powers BIGINT DEFAULT 0,
    PRIMARY KEY (GroupID, RoleID)
);

-- Group role members table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS os_groups_rolemembership (
    GroupID UUID,
    RoleID UUID,
    PrincipalID UUID,
    PRIMARY KEY (GroupID, RoleID, PrincipalID)
);

-- Group invites table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS os_groups_invites (
    InviteID UUID PRIMARY KEY,
    GroupID UUID NOT NULL,
    RoleID UUID NOT NULL,
    PrincipalID UUID NOT NULL,
    TMStamp INTEGER NOT NULL DEFAULT 0
);

-- Group notices table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS os_groups_notices (
    GroupID UUID,
    NoticeID UUID,
    TMStamp INTEGER NOT NULL,
    FromName VARCHAR(255) NOT NULL,
    Subject VARCHAR(255) NOT NULL,
    Message TEXT NOT NULL,
    HasAttachment INTEGER NOT NULL DEFAULT 0,
    AttachmentType INTEGER NOT NULL DEFAULT 0,
    AttachmentName VARCHAR(128) NOT NULL DEFAULT '',
    AttachmentItemID UUID NOT NULL,
    AttachmentOwnerID UUID NOT NULL,
    PRIMARY KEY (GroupID, NoticeID)
);

-- Estate settings table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS estate_settings (
    EstateID INTEGER PRIMARY KEY,
    EstateName VARCHAR(64) DEFAULT '',
    AbuseEmailToEstateOwner BOOLEAN DEFAULT FALSE,
    DenyAnonymous BOOLEAN DEFAULT FALSE,
    ResetHomeOnTeleport BOOLEAN DEFAULT FALSE,
    FixedSun BOOLEAN DEFAULT FALSE,
    DenyTransacted BOOLEAN DEFAULT FALSE,
    BlockDwell BOOLEAN DEFAULT FALSE,
    DenyIdentified BOOLEAN DEFAULT FALSE,
    AllowVoice BOOLEAN DEFAULT TRUE,
    UseGlobalTime BOOLEAN DEFAULT TRUE,
    PricePerMeter INTEGER DEFAULT 1,
    TaxFree BOOLEAN DEFAULT FALSE,
    AllowDirectTeleport BOOLEAN DEFAULT TRUE,
    RedirectGridX INTEGER DEFAULT 0,
    RedirectGridY INTEGER DEFAULT 0,
    ParentEstateID INTEGER DEFAULT 1,
    SunPosition DOUBLE PRECISION DEFAULT 0.0,
    EstateSkipScripts BOOLEAN DEFAULT FALSE,
    BillableFactor DOUBLE PRECISION DEFAULT 1.0,
    PublicAccess BOOLEAN DEFAULT TRUE,
    AbuseEmail VARCHAR(255) DEFAULT '',
    EstateOwner UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    DenyMinors BOOLEAN DEFAULT FALSE,
    AllowLandmark BOOLEAN DEFAULT TRUE,
    AllowParcelChanges BOOLEAN DEFAULT TRUE,
    AllowSetHome BOOLEAN DEFAULT TRUE
);

-- Estate map table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS estate_map (
    RegionID UUID PRIMARY KEY,
    EstateID INTEGER NOT NULL
);

-- Estate users table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS estate_users (
    EstateID INTEGER,
    uuid UUID,
    PRIMARY KEY (EstateID, uuid)
);

-- Estate groups table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS estate_groups (
    EstateID INTEGER,
    uuid UUID,
    PRIMARY KEY (EstateID, uuid)
);

-- Estate managers table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS estate_managers (
    EstateID INTEGER,
    uuid UUID,
    PRIMARY KEY (EstateID, uuid)
);

-- Estate bans table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS estate_ban (
    EstateID INTEGER,
    bannedUUID UUID,
    bannedIp VARCHAR(16),
    bannedIpHostMask VARCHAR(16),
    bannedNameMask VARCHAR(64),
    PRIMARY KEY (EstateID, bannedUUID)
);

-- Authentication table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS auth (
    UUID UUID PRIMARY KEY,
    passwordHash VARCHAR(32),
    passwordSalt VARCHAR(32),
    webLoginKey UUID,
    accountType VARCHAR(32) DEFAULT 'UserAccount'
);

-- Avatar appearances table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS avatarappearances (
    Owner UUID PRIMARY KEY,
    Serial INTEGER DEFAULT 0,
    Visual_Params BYTEA,
    Texture BYTEA,
    Avatar_Height DOUBLE PRECISION DEFAULT 0,
    Body_Item UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Body_Asset UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Skin_Item UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Skin_Asset UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Hair_Item UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Hair_Asset UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Eyes_Item UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Eyes_Asset UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Shirt_Item UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Shirt_Asset UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Pants_Item UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Pants_Asset UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Shoes_Item UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Shoes_Asset UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Socks_Item UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Socks_Asset UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Jacket_Item UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Jacket_Asset UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Gloves_Item UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Gloves_Asset UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Undershirt_Item UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Undershirt_Asset UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Underpants_Item UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Underpants_Asset UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Skirt_Item UUID DEFAULT '00000000-0000-0000-0000-000000000000',
    Skirt_Asset UUID DEFAULT '00000000-0000-0000-0000-000000000000'
);

-- Avatar attachments table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS avatarattachments (
    UUID UUID,
    attachpoint INTEGER,
    item UUID,
    asset UUID,
    PRIMARY KEY (UUID, attachpoint)
);

-- Mute list table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS mute (
    agentID UUID NOT NULL,
    muteID UUID NOT NULL,
    muteName VARCHAR(64) NOT NULL,
    muteType INTEGER NOT NULL DEFAULT 1,
    muteFlags INTEGER NOT NULL DEFAULT 0,
    mutestamp INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (agentID, muteID, muteName)
);

-- Offline messages table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS im_offline (
    ID SERIAL PRIMARY KEY,
    PrincipalID UUID NOT NULL,
    Message TEXT NOT NULL,
    TMStamp INTEGER NOT NULL
);

-- Migrations table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS migrations (
    name VARCHAR(100) PRIMARY KEY,
    version INTEGER NOT NULL
);

-- Profile data table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS classifieds (
    classifieduuid UUID PRIMARY KEY,
    creatoruuid UUID NOT NULL,
    creationdate INTEGER NOT NULL,
    expirationdate INTEGER NOT NULL,
    category VARCHAR(20) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    parceluuid UUID NOT NULL,
    parentestate INTEGER NOT NULL,
    snapshotuuid UUID NOT NULL,
    simname VARCHAR(255) NOT NULL,
    posglobal VARCHAR(255) NOT NULL,
    parcelname VARCHAR(255) NOT NULL,
    classifiedflags INTEGER NOT NULL,
    priceforlisting INTEGER NOT NULL
);

-- User picks table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS userpicks (
    pickuuid UUID PRIMARY KEY,
    creatoruuid UUID NOT NULL,
    toppick BOOLEAN NOT NULL,
    parceluuid UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    snapshotuuid UUID NOT NULL,
    user VARCHAR(255) NOT NULL,
    originalname VARCHAR(255) NOT NULL,
    simname VARCHAR(255) NOT NULL,
    posglobal VARCHAR(255) NOT NULL,
    sortorder INTEGER NOT NULL,
    enabled BOOLEAN NOT NULL,
    gatekeeper VARCHAR(255)
);

-- User notes table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS usernotes (
    useruuid UUID NOT NULL,
    targetuuid UUID NOT NULL,
    notes TEXT NOT NULL,
    PRIMARY KEY (useruuid, targetuuid)
);

-- User settings table (OpenSim compatible)
CREATE TABLE IF NOT EXISTS usersettings (
    useruuid UUID PRIMARY KEY,
    imviaemail BOOLEAN NOT NULL,
    visible BOOLEAN NOT NULL,
    email VARCHAR(254) NOT NULL
);

-- Create foreign key constraints for data integrity
ALTER TABLE estate_map ADD CONSTRAINT fk_estate_map_estate 
    FOREIGN KEY (EstateID) REFERENCES estate_settings(EstateID);

ALTER TABLE estate_users ADD CONSTRAINT fk_estate_users_estate 
    FOREIGN KEY (EstateID) REFERENCES estate_settings(EstateID);

ALTER TABLE estate_groups ADD CONSTRAINT fk_estate_groups_estate 
    FOREIGN KEY (EstateID) REFERENCES estate_settings(EstateID);

ALTER TABLE estate_managers ADD CONSTRAINT fk_estate_managers_estate 
    FOREIGN KEY (EstateID) REFERENCES estate_settings(EstateID);

ALTER TABLE estate_ban ADD CONSTRAINT fk_estate_ban_estate 
    FOREIGN KEY (EstateID) REFERENCES estate_settings(EstateID);

-- Create additional indexes for performance
CREATE INDEX IF NOT EXISTS idx_land_access_list_land ON landaccesslist(LandUUID);
CREATE INDEX IF NOT EXISTS idx_presence_session ON presence(SessionID);
CREATE INDEX IF NOT EXISTS idx_griduser_region ON griduser(LastRegionID);
CREATE INDEX IF NOT EXISTS idx_friends_friend ON friends(Friend);
CREATE INDEX IF NOT EXISTS idx_groups_name ON os_groups_groups(Name);
CREATE INDEX IF NOT EXISTS idx_group_membership_principal ON os_groups_membership(PrincipalID);
CREATE INDEX IF NOT EXISTS idx_estate_map_estate ON estate_map(EstateID);
CREATE INDEX IF NOT EXISTS idx_avatar_attachments_item ON avatarattachments(item);
CREATE INDEX IF NOT EXISTS idx_im_offline_principal ON im_offline(PrincipalID);
CREATE INDEX IF NOT EXISTS idx_classifieds_creator ON classifieds(creatoruuid);
CREATE INDEX IF NOT EXISTS idx_userpicks_creator ON userpicks(creatoruuid);

-- Comments for future AI integration
COMMENT ON TABLE regions IS 'OpenSim region definitions - base for AI-enhanced world generation';
COMMENT ON TABLE user_accounts IS 'OpenSim user accounts - extended with AI behavior tracking';
COMMENT ON TABLE primitives IS 'OpenSim primitive objects - enhanced with AI-driven behaviors';
COMMENT ON TABLE assets IS 'OpenSim asset storage - supports AI-generated content';
COMMENT ON TABLE land IS 'OpenSim land parcels - managed by AI economy systems';

-- Version tracking for schema evolution
INSERT INTO migrations (name, version) VALUES ('opensim_compatibility', 1) 
ON CONFLICT (name) DO UPDATE SET version = EXCLUDED.version;