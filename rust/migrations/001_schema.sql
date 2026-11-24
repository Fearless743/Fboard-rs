/*M!999999\- enable the sandbox mode */ 
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*M!100616 SET @OLD_NOTE_VERBOSITY=@@NOTE_VERBOSITY, NOTE_VERBOSITY=0 */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `failed_jobs` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `connection` text NOT NULL,
  `queue` text NOT NULL,
  `payload` longtext NOT NULL,
  `exception` longtext NOT NULL,
  `failed_at` timestamp NOT NULL DEFAULT current_timestamp(),
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `migrations` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `migration` varchar(255) NOT NULL,
  `batch` int(11) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=59 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `personal_access_tokens` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `tokenable_type` varchar(255) NOT NULL,
  `tokenable_id` bigint(20) unsigned NOT NULL,
  `name` varchar(255) NOT NULL,
  `token` varchar(64) NOT NULL,
  `abilities` text DEFAULT NULL,
  `last_used_at` timestamp NULL DEFAULT NULL,
  `expires_at` timestamp NULL DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `personal_access_tokens_token_unique` (`token`),
  KEY `personal_access_tokens_tokenable_type_tokenable_id_index` (`tokenable_type`,`tokenable_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_admin_audit_log` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `admin_id` bigint(20) unsigned NOT NULL,
  `action` varchar(64) NOT NULL COMMENT 'Action identifier e.g. user.update',
  `method` varchar(10) NOT NULL,
  `uri` varchar(512) NOT NULL,
  `request_data` text DEFAULT NULL,
  `ip` varchar(128) DEFAULT NULL,
  `created_at` int(10) unsigned NOT NULL,
  `updated_at` int(10) unsigned NOT NULL,
  PRIMARY KEY (`id`),
  KEY `v2_admin_audit_log_admin_id_index` (`admin_id`),
  KEY `v2_admin_audit_log_action_index` (`action`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_cert_templates` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(128) NOT NULL COMMENT '模板名称',
  `description` varchar(255) DEFAULT NULL COMMENT '模板描述',
  `cert_content` longtext NOT NULL COMMENT '证书内容 PEM',
  `key_content` longtext NOT NULL COMMENT '私钥内容 PEM',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  `pinyin_index` text DEFAULT NULL COMMENT '拼音搜索索引（全拼 + 首字母缩写，由 HasPinyinSearch trait 自动维护）',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_commission_log` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `invite_user_id` int(11) NOT NULL,
  `user_id` int(11) NOT NULL,
  `trade_no` char(36) NOT NULL,
  `order_amount` int(11) NOT NULL,
  `get_amount` int(11) NOT NULL,
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `v2_commission_log_created_at_index` (`created_at`),
  KEY `v2_commission_log_get_amount_index` (`get_amount`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_coupon` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `code` varchar(255) NOT NULL,
  `name` varchar(255) NOT NULL,
  `type` int(11) NOT NULL,
  `value` int(11) NOT NULL,
  `show` tinyint(1) NOT NULL DEFAULT 0,
  `limit_use` int(11) DEFAULT NULL,
  `limit_use_with_user` int(11) DEFAULT NULL,
  `limit_plan_ids` varchar(255) DEFAULT NULL,
  `limit_period` varchar(255) DEFAULT NULL,
  `started_at` int(11) NOT NULL,
  `ended_at` int(11) NOT NULL,
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  `pinyin_index` text DEFAULT NULL COMMENT '拼音搜索索引（全拼 + 首字母缩写，由 HasPinyinSearch trait 自动维护）',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_gift_card_code` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `template_id` int(11) NOT NULL COMMENT '模板ID',
  `code` varchar(32) NOT NULL COMMENT '兑换码',
  `batch_id` varchar(32) DEFAULT NULL COMMENT '批次ID',
  `status` tinyint(4) NOT NULL DEFAULT 0 COMMENT '状态：0未使用 1已使用 2已过期 3已禁用',
  `user_id` int(11) DEFAULT NULL COMMENT '使用用户ID',
  `used_at` int(11) DEFAULT NULL COMMENT '使用时间',
  `expires_at` int(11) DEFAULT NULL COMMENT '过期时间',
  `actual_rewards` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL COMMENT '实际获得的奖励(用于盲盒等)' CHECK (json_valid(`actual_rewards`)),
  `usage_count` int(11) NOT NULL DEFAULT 0 COMMENT '使用次数(分享卡)',
  `max_usage` int(11) NOT NULL DEFAULT 1 COMMENT '最大使用次数',
  `metadata` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL COMMENT '额外数据' CHECK (json_valid(`metadata`)),
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `v2_gift_card_code_code_unique` (`code`),
  KEY `idx_gift_code_template_id` (`template_id`),
  KEY `idx_gift_code_status` (`status`),
  KEY `idx_gift_code_user_id` (`user_id`),
  KEY `idx_gift_code_batch_id` (`batch_id`),
  KEY `idx_gift_code_expires_at` (`expires_at`),
  KEY `idx_gift_code_lookup` (`code`,`status`,`expires_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_gift_card_template` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL COMMENT '礼品卡名称',
  `description` text DEFAULT NULL COMMENT '礼品卡描述',
  `type` tinyint(4) NOT NULL COMMENT '卡片类型：1余额 2有效期 3流量 4重置包 5套餐 6组合 7盲盒 8任务 9等级 10节日',
  `status` tinyint(4) NOT NULL DEFAULT 1 COMMENT '状态：0禁用 1启用',
  `conditions` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL COMMENT '使用条件配置' CHECK (json_valid(`conditions`)),
  `rewards` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL COMMENT '奖励配置' CHECK (json_valid(`rewards`)),
  `limits` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL COMMENT '限制条件' CHECK (json_valid(`limits`)),
  `special_config` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL COMMENT '特殊配置(节日时间、等级倍率等)' CHECK (json_valid(`special_config`)),
  `icon` varchar(255) DEFAULT NULL COMMENT '卡片图标',
  `background_image` varchar(255) DEFAULT NULL COMMENT '背景图片URL',
  `theme_color` varchar(7) NOT NULL DEFAULT '#1890ff' COMMENT '主题色',
  `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
  `admin_id` int(11) NOT NULL COMMENT '创建管理员ID',
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  `pinyin_index` text DEFAULT NULL COMMENT '拼音搜索索引（全拼 + 首字母缩写，由 HasPinyinSearch trait 自动维护）',
  PRIMARY KEY (`id`),
  KEY `idx_gift_template_type_status` (`type`,`status`),
  KEY `idx_gift_template_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_gift_card_usage` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `code_id` int(11) NOT NULL COMMENT '兑换码ID',
  `template_id` int(11) NOT NULL COMMENT '模板ID',
  `user_id` int(11) NOT NULL COMMENT '使用用户ID',
  `invite_user_id` int(11) DEFAULT NULL COMMENT '邀请人ID',
  `rewards_given` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL COMMENT '实际发放的奖励' CHECK (json_valid(`rewards_given`)),
  `invite_rewards` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL COMMENT '邀请人获得的奖励' CHECK (json_valid(`invite_rewards`)),
  `user_level_at_use` int(11) DEFAULT NULL COMMENT '使用时用户等级',
  `plan_id_at_use` int(11) DEFAULT NULL COMMENT '使用时用户套餐ID',
  `multiplier_applied` decimal(3,2) NOT NULL DEFAULT 1.00 COMMENT '应用的倍率',
  `ip_address` varchar(45) DEFAULT NULL COMMENT '使用IP地址',
  `user_agent` text DEFAULT NULL COMMENT '用户代理',
  `notes` text DEFAULT NULL COMMENT '备注',
  `created_at` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_gift_usage_code_id` (`code_id`),
  KEY `idx_gift_usage_template_id` (`template_id`),
  KEY `idx_gift_usage_user_id` (`user_id`),
  KEY `idx_gift_usage_invite_user_id` (`invite_user_id`),
  KEY `idx_gift_usage_created_at` (`created_at`),
  KEY `idx_gift_usage_user_usage` (`user_id`,`created_at`),
  KEY `idx_gift_usage_template_stats` (`template_id`,`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_invite_code` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `user_id` int(11) NOT NULL,
  `code` char(32) NOT NULL,
  `status` tinyint(1) NOT NULL DEFAULT 0,
  `pv` int(11) NOT NULL DEFAULT 0,
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_knowledge` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `language` char(5) NOT NULL COMMENT '語言',
  `category` varchar(255) NOT NULL COMMENT '分類名',
  `title` varchar(255) NOT NULL COMMENT '標題',
  `body` text NOT NULL COMMENT '內容',
  `sort` int(11) DEFAULT NULL COMMENT '排序',
  `show` tinyint(1) NOT NULL DEFAULT 0 COMMENT '顯示',
  `created_at` int(11) NOT NULL COMMENT '創建時間',
  `updated_at` int(11) NOT NULL COMMENT '更新時間',
  `pinyin_index` text DEFAULT NULL COMMENT '拼音搜索索引（全拼 + 首字母缩写，由 HasPinyinSearch trait 自动维护）',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_log` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `title` text NOT NULL,
  `level` varchar(11) DEFAULT NULL,
  `host` varchar(255) DEFAULT NULL,
  `uri` varchar(255) NOT NULL,
  `method` varchar(11) NOT NULL,
  `data` text DEFAULT NULL,
  `ip` varchar(128) DEFAULT NULL,
  `context` text DEFAULT NULL,
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_mail_log` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `email` varchar(64) NOT NULL,
  `subject` varchar(255) NOT NULL,
  `template_name` varchar(255) NOT NULL,
  `error` text DEFAULT NULL,
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_mail_templates` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(64) NOT NULL,
  `subject` varchar(255) NOT NULL,
  `content` longtext NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `v2_mail_templates_name_unique` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_network_settings_templates` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(128) NOT NULL COMMENT '模板名称',
  `description` varchar(255) DEFAULT NULL COMMENT '模板描述',
  `network` varchar(32) DEFAULT NULL COMMENT '关联传输协议，如 ws/grpc/tcp，空为通用',
  `settings` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL COMMENT 'network_settings JSON 对象' CHECK (json_valid(`settings`)),
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `v2_network_settings_templates_network_index` (`network`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_notice` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `sort` int(11) DEFAULT NULL,
  `title` varchar(255) NOT NULL,
  `content` text NOT NULL,
  `show` tinyint(1) NOT NULL DEFAULT 0,
  `img_url` varchar(255) DEFAULT NULL,
  `tags` varchar(255) DEFAULT NULL,
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  `pinyin_index` text DEFAULT NULL COMMENT '拼音搜索索引（全拼 + 首字母缩写，由 HasPinyinSearch trait 自动维护）',
  PRIMARY KEY (`id`),
  KEY `v2_notice_sort_index` (`sort`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_order` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `invite_user_id` int(11) DEFAULT NULL,
  `user_id` int(11) NOT NULL,
  `plan_id` int(11) NOT NULL,
  `coupon_id` int(11) DEFAULT NULL,
  `payment_id` int(11) DEFAULT NULL,
  `type` int(11) NOT NULL COMMENT '1新购2续费3升级',
  `period` varchar(255) NOT NULL,
  `trade_no` varchar(36) NOT NULL,
  `callback_no` varchar(255) DEFAULT NULL,
  `total_amount` int(11) NOT NULL,
  `handling_amount` int(11) DEFAULT NULL,
  `discount_amount` int(11) DEFAULT NULL,
  `surplus_amount` int(11) DEFAULT NULL COMMENT '剩余价值',
  `surplus_credit` int(11) DEFAULT NULL COMMENT '退款金额',
  `balance_amount` int(11) DEFAULT NULL COMMENT '使用余额',
  `surplus_order_ids` text DEFAULT NULL COMMENT '折抵订单',
  `status` int(11) NOT NULL DEFAULT 0 COMMENT '0待支付1开通中2已取消3已完成4已折抵',
  `commission_status` int(11) NOT NULL DEFAULT 0 COMMENT '0待确认1发放中2有效3无效',
  `commission_balance` int(11) NOT NULL DEFAULT 0,
  `actual_commission_balance` int(11) DEFAULT NULL COMMENT '实际支付佣金',
  `paid_at` int(11) DEFAULT NULL,
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `trade_no` (`trade_no`),
  KEY `v2_order_created_at_index` (`created_at`),
  KEY `v2_order_status_index` (`status`),
  KEY `v2_order_total_amount_index` (`total_amount`),
  KEY `v2_order_commission_status_index` (`commission_status`),
  KEY `v2_order_invite_user_id_index` (`invite_user_id`),
  KEY `v2_order_commission_balance_index` (`commission_balance`),
  KEY `v2_order_updated_at_index` (`updated_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_payment` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `uuid` char(32) NOT NULL,
  `payment` varchar(16) NOT NULL,
  `name` varchar(255) NOT NULL,
  `icon` text DEFAULT NULL,
  `config` text NOT NULL,
  `notify_domain` varchar(128) DEFAULT NULL,
  `handling_fee_fixed` int(11) DEFAULT NULL,
  `handling_fee_percent` decimal(5,2) DEFAULT NULL,
  `enable` tinyint(1) NOT NULL DEFAULT 0,
  `sort` int(11) DEFAULT NULL,
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  `pinyin_index` text DEFAULT NULL COMMENT '拼音搜索索引（全拼 + 首字母缩写，由 HasPinyinSearch trait 自动维护）',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_plan` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `group_id` int(10) unsigned DEFAULT NULL,
  `transfer_enable` bigint(20) unsigned DEFAULT NULL COMMENT 'Transfer limit in bytes',
  `name` varchar(255) NOT NULL,
  `prices` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL COMMENT 'Store different duration prices and reset traffic price' CHECK (json_valid(`prices`)),
  `sell` tinyint(1) NOT NULL DEFAULT 0 COMMENT 'is sell',
  `speed_limit` int(10) unsigned DEFAULT NULL COMMENT 'Speed limit in Mbps, 0 for unlimited',
  `device_limit` int(10) unsigned DEFAULT NULL,
  `show` tinyint(1) NOT NULL DEFAULT 0,
  `sort` int(11) DEFAULT NULL,
  `renew` tinyint(1) NOT NULL DEFAULT 1,
  `content` text DEFAULT NULL,
  `tags` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL CHECK (json_valid(`tags`)),
  `reset_traffic_method` int(11) DEFAULT 0 COMMENT '重置流量方式:null跟随系统设置、0每月1号、1按月重置、2不重置、3每年1月1日、4按年重置',
  `capacity_limit` int(10) unsigned DEFAULT 0 COMMENT '0 for unlimited',
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  `pinyin_index` text DEFAULT NULL COMMENT '拼音搜索索引（全拼 + 首字母缩写，由 HasPinyinSearch trait 自动维护）',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_plugins` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `code` varchar(255) NOT NULL,
  `type` varchar(20) NOT NULL DEFAULT 'feature' COMMENT '插件类型：feature功能性，payment支付型',
  `version` varchar(50) NOT NULL,
  `is_enabled` tinyint(1) NOT NULL DEFAULT 0,
  `config` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL CHECK (json_valid(`config`)),
  `installed_at` timestamp NULL DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `v2_plugins_code_unique` (`code`),
  KEY `v2_plugins_type_is_enabled_index` (`type`,`is_enabled`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_server` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `type` varchar(255) NOT NULL COMMENT 'Server Type',
  `code` varchar(255) DEFAULT NULL COMMENT 'Server Spectific Key',
  `parent_id` int(10) unsigned DEFAULT NULL COMMENT 'Parent Server ID',
  `machine_id` bigint(20) unsigned DEFAULT NULL,
  `group_ids` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL COMMENT 'Group ID' CHECK (json_valid(`group_ids`)),
  `route_ids` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL COMMENT 'Route ID' CHECK (json_valid(`route_ids`)),
  `name` varchar(255) NOT NULL COMMENT 'Server Name',
  `rate` decimal(8,2) NOT NULL COMMENT 'Traffic Rate',
  `transfer_enable` bigint(20) DEFAULT NULL COMMENT 'Traffic limit , 0 or null=no limit',
  `u` bigint(20) NOT NULL DEFAULT 0 COMMENT 'upload traffic',
  `d` bigint(20) NOT NULL DEFAULT 0 COMMENT 'donwload traffic',
  `rate_time_enable` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否启用动态倍率',
  `rate_time_ranges` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL COMMENT '动态倍率规则' CHECK (json_valid(`rate_time_ranges`)),
  `tags` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL COMMENT 'Server Tags' CHECK (json_valid(`tags`)),
  `host` varchar(255) NOT NULL COMMENT 'Server Host',
  `port` varchar(255) NOT NULL COMMENT 'Client Port',
  `server_port` int(11) NOT NULL COMMENT 'Server Port',
  `protocol_settings` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL CHECK (json_valid(`protocol_settings`)),
  `custom_outbounds` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL CHECK (json_valid(`custom_outbounds`)),
  `custom_routes` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL CHECK (json_valid(`custom_routes`)),
  `cert_config` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL CHECK (json_valid(`cert_config`)),
  `show` tinyint(1) NOT NULL DEFAULT 0 COMMENT 'Show in List',
  `enabled` tinyint(1) DEFAULT 1,
  `sort` int(10) unsigned DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  `pinyin_index` text DEFAULT NULL COMMENT '拼音搜索索引（全拼 + 首字母缩写，由 HasPinyinSearch trait 自动维护）',
  PRIMARY KEY (`id`),
  UNIQUE KEY `v2_server_type_code_unique` (`type`,`code`),
  KEY `v2_server_sort_index` (`sort`),
  KEY `v2_server_machine_id_foreign` (`machine_id`),
  CONSTRAINT `v2_server_machine_id_foreign` FOREIGN KEY (`machine_id`) REFERENCES `v2_server_machine` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_server_group` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_server_machine` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `token` varchar(64) NOT NULL,
  `notes` text DEFAULT NULL,
  `is_active` tinyint(1) NOT NULL DEFAULT 1,
  `last_seen_at` int(10) unsigned DEFAULT NULL,
  `load_status` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL CHECK (json_valid(`load_status`)),
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  `pinyin_index` text DEFAULT NULL COMMENT '拼音搜索索引（全拼 + 首字母缩写，由 HasPinyinSearch trait 自动维护）',
  PRIMARY KEY (`id`),
  UNIQUE KEY `v2_server_machine_token_unique` (`token`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_server_machine_load_history` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `machine_id` bigint(20) unsigned NOT NULL,
  `cpu` double NOT NULL DEFAULT 0,
  `mem_total` bigint(20) unsigned NOT NULL DEFAULT 0,
  `mem_used` bigint(20) unsigned NOT NULL DEFAULT 0,
  `disk_total` bigint(20) unsigned NOT NULL DEFAULT 0,
  `disk_used` bigint(20) unsigned NOT NULL DEFAULT 0,
  `net_in_speed` double DEFAULT NULL,
  `net_out_speed` double DEFAULT NULL,
  `recorded_at` int(10) unsigned NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `v2_server_machine_load_history_machine_id_recorded_at_index` (`machine_id`,`recorded_at`),
  CONSTRAINT `v2_server_machine_load_history_machine_id_foreign` FOREIGN KEY (`machine_id`) REFERENCES `v2_server_machine` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_server_route` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `remarks` varchar(255) NOT NULL,
  `match` text NOT NULL,
  `action` varchar(11) NOT NULL,
  `action_value` varchar(255) DEFAULT NULL,
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_settings` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `group` varchar(255) DEFAULT NULL COMMENT '设置分组',
  `type` varchar(255) DEFAULT NULL COMMENT '设置类型',
  `name` varchar(255) NOT NULL COMMENT '设置名称',
  `value` mediumtext DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `v2_settings_name_unique` (`name`),
  KEY `idx_setting_name` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_stat` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `record_at` int(11) NOT NULL,
  `record_type` char(1) NOT NULL,
  `order_count` int(11) NOT NULL COMMENT '订单数量',
  `order_total` int(11) NOT NULL COMMENT '订单合计',
  `commission_count` int(11) NOT NULL,
  `commission_total` int(11) NOT NULL COMMENT '佣金合计',
  `paid_count` int(11) NOT NULL,
  `paid_total` int(11) NOT NULL,
  `register_count` int(11) NOT NULL,
  `invite_count` int(11) NOT NULL,
  `transfer_used_total` varchar(32) NOT NULL,
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `v2_stat_record_at_unique` (`record_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_stat_server` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `server_id` int(11) NOT NULL COMMENT '节点id',
  `server_type` char(11) NOT NULL COMMENT '节点类型',
  `u` bigint(20) NOT NULL,
  `d` bigint(20) NOT NULL,
  `record_type` char(1) NOT NULL COMMENT 'd day m month',
  `record_at` int(11) NOT NULL COMMENT '记录时间',
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `server_id_server_type_record_at` (`server_id`,`server_type`,`record_at`),
  KEY `server_id` (`server_id`),
  KEY `record_at` (`record_at`),
  KEY `v2_stat_server_server_id_index` (`server_id`),
  KEY `v2_stat_server_record_at_index` (`record_at`),
  KEY `v2_stat_server_u_index` (`u`),
  KEY `v2_stat_server_d_index` (`d`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_stat_user` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `user_id` int(11) NOT NULL,
  `server_rate` decimal(10,2) NOT NULL,
  `u` bigint(20) NOT NULL,
  `d` bigint(20) NOT NULL,
  `record_type` char(2) NOT NULL,
  `record_at` int(11) NOT NULL,
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `server_rate_user_id_record_at` (`server_rate`,`user_id`,`record_at`),
  KEY `v2_stat_user_user_id_server_rate_record_at_index` (`user_id`,`server_rate`,`record_at`),
  KEY `v2_stat_user_u_index` (`u`),
  KEY `v2_stat_user_d_index` (`d`),
  KEY `idx_stat_user_record_user` (`record_at`,`user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_subscribe_templates` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL COMMENT 'Template key, e.g. singbox, clash',
  `content` mediumtext DEFAULT NULL COMMENT 'Template content',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `v2_subscribe_templates_name_unique` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_ticket` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `user_id` int(11) NOT NULL,
  `subject` varchar(255) NOT NULL,
  `level` int(11) NOT NULL,
  `status` int(11) NOT NULL DEFAULT 0 COMMENT '0:已开启 1:已关闭',
  `reply_status` int(11) NOT NULL DEFAULT 0 COMMENT '0:待回复 1:已回复',
  `last_reply_user_id` int(11) DEFAULT NULL,
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  `pinyin_index` text DEFAULT NULL COMMENT '拼音搜索索引（全拼 + 首字母缩写，由 HasPinyinSearch trait 自动维护）',
  PRIMARY KEY (`id`),
  KEY `v2_ticket_status_index` (`status`),
  KEY `v2_ticket_created_at_index` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_ticket_message` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `user_id` int(11) NOT NULL,
  `ticket_id` int(11) NOT NULL,
  `message` text NOT NULL,
  `is_bot` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否为机器人自动回复',
  `recalled_at` int(11) DEFAULT NULL COMMENT '撤回时间，非空表示消息已撤回',
  `edited_at` int(11) DEFAULT NULL COMMENT '最后编辑时间',
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_traffic_reset_logs` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `user_id` bigint(20) NOT NULL COMMENT '用户ID',
  `reset_type` varchar(50) NOT NULL COMMENT '重置类型',
  `reset_time` timestamp NOT NULL COMMENT '重置时间',
  `old_upload` bigint(20) NOT NULL DEFAULT 0 COMMENT '重置前上传流量',
  `old_download` bigint(20) NOT NULL DEFAULT 0 COMMENT '重置前下载流量',
  `old_total` bigint(20) NOT NULL DEFAULT 0 COMMENT '重置前总流量',
  `new_upload` bigint(20) NOT NULL DEFAULT 0 COMMENT '重置后上传流量',
  `new_download` bigint(20) NOT NULL DEFAULT 0 COMMENT '重置后下载流量',
  `new_total` bigint(20) NOT NULL DEFAULT 0 COMMENT '重置后总流量',
  `trigger_source` varchar(50) NOT NULL COMMENT '触发来源',
  `metadata` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL COMMENT '额外元数据' CHECK (json_valid(`metadata`)),
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_reset_time` (`reset_time`),
  KEY `idx_user_reset_time` (`user_id`,`reset_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_user` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `invite_user_id` int(11) DEFAULT NULL,
  `telegram_id` bigint(20) DEFAULT NULL,
  `email` varchar(64) NOT NULL,
  `password` varchar(64) NOT NULL,
  `password_algo` char(10) DEFAULT NULL,
  `password_salt` char(10) DEFAULT NULL,
  `balance` int(11) NOT NULL DEFAULT 0,
  `discount` int(11) DEFAULT NULL,
  `commission_type` tinyint(4) NOT NULL DEFAULT 0 COMMENT '0: system 1: period 2: onetime',
  `commission_rate` int(11) DEFAULT NULL,
  `commission_balance` int(11) NOT NULL DEFAULT 0,
  `t` int(11) NOT NULL DEFAULT 0,
  `u` bigint(20) NOT NULL DEFAULT 0,
  `d` bigint(20) NOT NULL DEFAULT 0,
  `transfer_enable` bigint(20) NOT NULL DEFAULT 0,
  `banned` tinyint(1) NOT NULL DEFAULT 0,
  `is_admin` tinyint(1) NOT NULL DEFAULT 0,
  `last_login_at` int(11) DEFAULT NULL,
  `is_staff` tinyint(1) NOT NULL DEFAULT 0,
  `last_login_ip` varchar(45) DEFAULT NULL,
  `register_ip` varchar(45) DEFAULT NULL COMMENT '注册 IP',
  `uuid` varchar(36) NOT NULL,
  `group_id` int(11) DEFAULT NULL,
  `plan_id` int(11) DEFAULT NULL,
  `speed_limit` int(11) DEFAULT NULL,
  `remind_expire` tinyint(4) DEFAULT 1,
  `remind_traffic` tinyint(4) DEFAULT 1,
  `token` char(32) NOT NULL,
  `expired_at` bigint(20) DEFAULT 0,
  `next_reset_at` int(11) DEFAULT NULL COMMENT '下次流量重置时间',
  `last_reset_at` int(11) DEFAULT NULL COMMENT '上次流量重置时间',
  `reset_count` int(11) NOT NULL DEFAULT 0 COMMENT '流量重置次数',
  `device_limit` int(11) DEFAULT NULL,
  `online_count` int(11) DEFAULT NULL,
  `last_online_at` timestamp NULL DEFAULT NULL,
  `remarks` text DEFAULT NULL,
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  `usque_device_private` text DEFAULT NULL COMMENT 'usque 设备私钥 base64(SEC1 DER)',
  `usque_device_public` text DEFAULT NULL COMMENT 'usque 设备公钥 base64(PKIX DER)',
  PRIMARY KEY (`id`),
  UNIQUE KEY `email` (`email`),
  KEY `v2_user_u_d_expired_at_group_id_banned_transfer_enable_index` (`u`,`d`,`expired_at`,`group_id`,`banned`,`transfer_enable`),
  KEY `v2_user_t_index` (`t`),
  KEY `v2_user_online_count_index` (`online_count`),
  KEY `v2_user_created_at_index` (`created_at`),
  KEY `idx_next_reset_at` (`next_reset_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_user_login_log` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `user_id` bigint(20) unsigned NOT NULL COMMENT '用户 ID',
  `ip` varchar(45) NOT NULL COMMENT '登录 IP',
  `user_agent` varchar(512) DEFAULT NULL COMMENT 'User-Agent',
  `method` varchar(32) NOT NULL COMMENT '登录方式: password/register/mail_link',
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_user_login_log_user_id` (`user_id`),
  KEY `idx_user_login_log_user_created` (`user_id`,`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `v2_withdrawal` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `ticket_id` int(10) unsigned DEFAULT NULL,
  `user_id` int(11) NOT NULL,
  `withdraw_method` varchar(255) NOT NULL,
  `withdraw_account` varchar(255) NOT NULL,
  `amount` int(11) NOT NULL COMMENT '单位：分',
  `status` int(11) NOT NULL DEFAULT 0 COMMENT '0:待处理 1:已确认 2:已拒绝',
  `remark` text DEFAULT NULL,
  `operator_id` int(11) DEFAULT NULL COMMENT '处理人用户ID',
  `created_at` int(11) NOT NULL,
  `updated_at` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `v2_withdrawal_user_id_index` (`user_id`),
  KEY `v2_withdrawal_ticket_id_index` (`ticket_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE IF NOT EXISTS `withdrawal_messages` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `withdrawal_id` int(10) unsigned NOT NULL COMMENT '提现单ID',
  `user_id` int(10) unsigned NOT NULL COMMENT '发送者ID（管理员或客服）',
  `message` text NOT NULL COMMENT '消息内容',
  `is_admin` tinyint(3) unsigned NOT NULL DEFAULT 0 COMMENT '是否管理员发送',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `withdrawal_messages_withdrawal_id_index` (`withdrawal_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*M!100616 SET NOTE_VERBOSITY=@OLD_NOTE_VERBOSITY */;

