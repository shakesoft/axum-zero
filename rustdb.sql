/*
 Navicat Premium Dump SQL

 Source Server         : localhost
 Source Server Type    : MySQL
 Source Server Version : 80300 (8.3.0)
 Source Host           : 127.0.0.1:3306
 Source Schema         : rustdb

 Target Server Type    : MySQL
 Target Server Version : 80300 (8.3.0)
 File Encoding         : 65001

 Date: 26/02/2026 07:16:30
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for sys_dept
-- ----------------------------
DROP TABLE IF EXISTS `sys_dept`;
CREATE TABLE `sys_dept`  (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '部门id',
  `parent_id` bigint NOT NULL DEFAULT 0 COMMENT '父部门id',
  `ancestors` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '祖级列表',
  `dept_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '部门名称',
  `sort` int NOT NULL DEFAULT 0 COMMENT '显示顺序',
  `leader` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '负责人',
  `phone` varchar(11) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '联系电话',
  `email` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '邮箱',
  `status` tinyint NOT NULL DEFAULT 0 COMMENT '部门状态（0：停用，1:正常）',
  `del_flag` tinyint NOT NULL DEFAULT 1 COMMENT '删除标志（0代表删除 1代表存在）',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 36 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '部门表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_dept
-- ----------------------------
INSERT INTO `sys_dept` VALUES (1, 0, '0', '测试科技', 1, 'admin', '18613030352', '1002219331@qq.com', 1, 1, '2025-08-28 23:18:10', '2026-02-23 17:37:55');
INSERT INTO `sys_dept` VALUES (2, 1, '0,1', '深圳总公司', 1, '1', '1', 'xx@qq.com', 1, 1, '2025-08-28 23:18:10', '2025-08-28 23:18:10');
INSERT INTO `sys_dept` VALUES (3, 1, '0,1', '长沙分公司', 2, '1', '1', 'xx@qq.com', 1, 1, '2025-08-28 23:18:10', '2025-08-28 23:18:10');
INSERT INTO `sys_dept` VALUES (4, 2, '0,1,2', '研发部门', 1, '1', '1', 'xx@qq.com', 1, 1, '2025-08-28 23:18:10', '2025-08-28 23:18:10');
INSERT INTO `sys_dept` VALUES (5, 2, '0,1,2', '市场部门', 2, '1', '1', 'xx@qq.com', 1, 1, '2025-08-28 23:18:10', '2025-08-28 23:18:10');
INSERT INTO `sys_dept` VALUES (6, 2, '0,1,2', '测试部门', 3, '1', '1', 'xx@qq.com', 1, 1, '2025-08-28 23:18:10', '2025-08-28 23:18:10');
INSERT INTO `sys_dept` VALUES (7, 2, '0,1,2', '财务部门', 4, '1', '1', 'xx@qq.com', 1, 1, '2025-08-28 23:18:10', '2025-08-28 23:18:10');
INSERT INTO `sys_dept` VALUES (8, 2, '0,1,2', '运维部门', 5, '1', '1', 'xx@qq.com', 1, 1, '2025-08-28 23:18:10', '2025-08-28 23:18:10');
INSERT INTO `sys_dept` VALUES (9, 3, '0,1,3', '市场部门1', 6, '1', '1', 'xx@qq.com', 1, 1, '2025-08-28 23:18:10', '2025-08-28 23:18:10');
INSERT INTO `sys_dept` VALUES (10, 3, '0,1,3', '财务部门1', 1, '1', '1', 'xx@qq.com', 1, 1, '2025-08-28 23:18:10', '2025-08-28 23:18:10');
INSERT INTO `sys_dept` VALUES (11, 1, '0,1', '12312', 1, '12312', '13800138000', '12312312312312@qq.com', 1, 1, '2025-09-13 06:13:49', '2025-09-13 12:36:56');
INSERT INTO `sys_dept` VALUES (12, 1, '0,1', '123123343', 1, '12312', '13800138000', 'q34324@qq.com', 1, 1, '2025-09-13 06:14:20', '2025-09-13 12:36:43');
INSERT INTO `sys_dept` VALUES (13, 1, '0,1', '12312ss', 1, '123123', '13900139000', '136@qq.com', 1, 1, '2025-09-13 06:15:05', '2025-09-13 06:15:05');
INSERT INTO `sys_dept` VALUES (14, 1, '0,1', '13123', 1, '234234', '13700137000', '000@qq.com', 1, 1, '2025-09-13 06:16:10', '2025-09-13 06:16:09');
INSERT INTO `sys_dept` VALUES (15, 1, '0,1', '1231233', 1, '1312', '13800138000', '12312321@qq.com', 1, 1, '2025-09-13 07:36:08', '2025-09-13 07:36:07');
INSERT INTO `sys_dept` VALUES (16, 1, '0,1', '12312aa', 1, '12312', '13800138000', '2432@qq.com', 1, 1, '2025-09-13 09:35:27', '2025-09-13 09:35:26');
INSERT INTO `sys_dept` VALUES (17, 1, '0,1', '34322111', 1, '112', '13800138000', 'afdsa@qq.com', 1, 1, '2025-09-13 09:36:16', '2025-09-13 09:36:16');
INSERT INTO `sys_dept` VALUES (18, 1, '0,1', '4545', 1, '345345', '11111111111', '1@q.com', 1, 1, '2025-09-13 11:43:40', '2025-09-13 11:43:39');
INSERT INTO `sys_dept` VALUES (19, 1, '0,1', '3e4534', 1, '345345', '44444444444', '111@qq.com', 1, 1, '2025-09-13 11:55:28', '2025-09-13 11:55:28');
INSERT INTO `sys_dept` VALUES (20, 1, '0,1', '23423', 1, '23423', '33333333333', '23423@qq.com', 1, 1, '2025-09-13 12:01:29', '2025-09-13 12:01:28');
INSERT INTO `sys_dept` VALUES (21, 1, '0,1', '12312aaddd', 1, '12312', '88888888888', '12312321@qq.com', 1, 1, '2025-09-13 12:36:32', '2025-09-13 12:36:31');
INSERT INTO `sys_dept` VALUES (22, 1, '0,1', '12312ad', 1, '12312123', '33333333332', '31231@qq.com', 1, 1, '2025-09-13 12:38:53', '2025-09-13 12:38:53');
INSERT INTO `sys_dept` VALUES (23, 1, '0,1', '1231', 1, '12312', '99999999999', '1231231@qq.com', 1, 1, '2025-09-13 14:47:24', '2025-09-13 14:47:24');
INSERT INTO `sys_dept` VALUES (24, 1, '0,1', '12312ddeee', 1, '12321', '13800138000', '1312312@qq.com', 1, 1, '2025-09-17 06:12:47', '2025-09-17 06:12:46');
INSERT INTO `sys_dept` VALUES (25, 1, '0,1', '12321ss', 1, '21312321', '13800138000', '1@qq.com', 1, 1, '2025-09-17 06:14:45', '2025-09-17 06:14:44');
INSERT INTO `sys_dept` VALUES (26, 1, '0,1', '12321334', 1, '12321', '13800138000', '23@q.com', 1, 1, '2025-09-17 06:16:47', '2025-09-17 06:16:47');
INSERT INTO `sys_dept` VALUES (27, 1, '0,1', '121', 1, '1312312', '13800138000', '55@qq.com', 1, 1, '2025-09-17 06:21:36', '2025-09-17 06:21:35');
INSERT INTO `sys_dept` VALUES (28, 1, '0,1', '123123', 1123, '123', '11111111111', '11@11.com', 1, 1, '2025-09-17 09:16:27', '2025-09-17 09:16:27');
INSERT INTO `sys_dept` VALUES (29, 1, '0,1', '1231211w', 1, '12312', '13800138000', '138@qq.com', 1, 1, '2025-09-17 09:17:16', '2025-09-17 09:17:15');
INSERT INTO `sys_dept` VALUES (30, 1, '0,1', '123ss', 1, '1233', '13800138000', '12312@qq.com', 1, 1, '2025-12-30 07:24:49', '2025-12-30 07:24:48');
INSERT INTO `sys_dept` VALUES (31, 1, '0,1', '88542', 1, '58451', '13700137000', '1215@qq.com', 1, 1, '2025-12-30 07:26:55', '2025-12-30 07:26:54');
INSERT INTO `sys_dept` VALUES (32, 1, '0,1', '99665', 1, '99665', '13800138000', '996585@qq.com', 1, 1, '2025-12-30 07:31:34', '2025-12-30 07:31:33');
INSERT INTO `sys_dept` VALUES (33, 1, '0,1', '1112222', 1, '3333', '13800138000', '13522@qq.com', 1, 1, '2026-01-02 08:35:07', '2026-01-02 08:35:06');
INSERT INTO `sys_dept` VALUES (34, 1, '0,1', '12312322ed', 1, '123123', '13800138000', '1231233@qq.com', 1, 1, '2026-01-02 08:47:44', '2026-02-23 17:37:55');
INSERT INTO `sys_dept` VALUES (35, 1, '0,1', '22wweeqq', 1, 'aadd', '13800138000', 'dd23432@qq.com', 1, 1, '2026-01-05 11:54:40', '2026-01-05 11:54:39');

-- ----------------------------
-- Table structure for sys_dict_data
-- ----------------------------
DROP TABLE IF EXISTS `sys_dict_data`;
CREATE TABLE `sys_dict_data`  (
  `dict_code` bigint NOT NULL AUTO_INCREMENT COMMENT '字典编码',
  `dict_sort` int NOT NULL DEFAULT 0 COMMENT '字典排序',
  `dict_label` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '字典标签',
  `dict_value` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '字典键值',
  `dict_type` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '字典类型',
  `css_class` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '样式属性（其他样式扩展）',
  `list_class` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '表格回显样式',
  `is_default` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT 'N' COMMENT '是否默认（Y是 N否）',
  `status` tinyint NOT NULL DEFAULT 0 COMMENT '状态（0：停用，1:正常）',
  `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '备注',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
  PRIMARY KEY (`dict_code`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 6 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '字典数据表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_dict_data
-- ----------------------------
INSERT INTO `sys_dict_data` VALUES (1, 1, '男', '0', 'sys_user_sex', '1', '1', 'N', 1, '性别男', '2025-08-28 23:18:25', '2025-08-28 23:18:25');
INSERT INTO `sys_dict_data` VALUES (2, 2, '女', '1', 'sys_user_sex', '1', '1', 'N', 1, '性别女', '2025-08-28 23:18:25', '2025-08-28 23:18:25');
INSERT INTO `sys_dict_data` VALUES (3, 3, '未知', '2', 'sys_user_sex', '1', '1', 'N', 1, '性别未知', '2025-08-28 23:18:25', '2025-08-28 23:18:25');
INSERT INTO `sys_dict_data` VALUES (4, 1, '通知', '1', 'sys_notice_type', '1', '1', 'N', 1, '通知', '2025-08-28 23:18:25', '2025-08-28 23:18:25');
INSERT INTO `sys_dict_data` VALUES (5, 2, '公告', '2', 'sys_notice_type', '1', '1', 'N', 1, '公告', '2025-08-28 23:18:25', '2025-08-28 23:18:25');

-- ----------------------------
-- Table structure for sys_dict_type
-- ----------------------------
DROP TABLE IF EXISTS `sys_dict_type`;
CREATE TABLE `sys_dict_type`  (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '字典主键',
  `dict_name` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '字典名称',
  `dict_type` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '字典类型',
  `status` tinyint NOT NULL DEFAULT 0 COMMENT '状态（0：停用，1:正常）',
  `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '备注',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE INDEX `dict_type`(`dict_type` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 6 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '字典类型表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_dict_type
-- ----------------------------
INSERT INTO `sys_dict_type` VALUES (1, '用户性别', 'sys_user_sex', 1, '用户性别列表', '2025-08-28 23:18:33', '2025-08-28 23:18:33');
INSERT INTO `sys_dict_type` VALUES (2, '通知类型', 'sys_notice_type', 1, '通知类型列表', '2025-08-28 23:18:33', '2025-08-28 23:18:33');
INSERT INTO `sys_dict_type` VALUES (3, '案件状态', '测试', 1, '', '2025-08-29 07:11:39', '2025-08-29 07:11:39');
INSERT INTO `sys_dict_type` VALUES (4, '555', '666', 1, '', '2025-08-29 07:13:05', '2025-08-29 07:13:05');
INSERT INTO `sys_dict_type` VALUES (5, '555666', '6667777', 1, '', '2025-08-29 07:16:57', '2025-08-29 07:16:57');

-- ----------------------------
-- Table structure for sys_login_log
-- ----------------------------
DROP TABLE IF EXISTS `sys_login_log`;
CREATE TABLE `sys_login_log`  (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '访问ID',
  `login_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '登录账号',
  `ipaddr` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '登录IP地址',
  `login_location` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '登录地点',
  `platform` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '平台信息',
  `browser` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '浏览器类型',
  `version` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '浏览器版本',
  `os` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '操作系统',
  `arch` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '体系结构信息',
  `engine` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '渲染引擎信息',
  `engine_details` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '渲染引擎详细信息',
  `extra` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '其他信息（可选）',
  `status` tinyint NOT NULL DEFAULT 0 COMMENT '登录状态(0:失败,1:成功)',
  `msg` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '提示消息',
  `login_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '访问时间',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 91 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '系统访问记录' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_login_log
-- ----------------------------
INSERT INTO `sys_login_log` VALUES (1, 'jianzhu1', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '139.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2025-08-28 23:20:16');
INSERT INTO `sys_login_log` VALUES (2, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '139.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2025-08-28 23:20:34');
INSERT INTO `sys_login_log` VALUES (3, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '139.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2025-08-28 23:20:37');
INSERT INTO `sys_login_log` VALUES (4, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '139.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2025-08-28 23:20:39');
INSERT INTO `sys_login_log` VALUES (5, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '139.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2025-08-28 23:21:16');
INSERT INTO `sys_login_log` VALUES (6, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '139.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2025-08-28 23:39:59');
INSERT INTO `sys_login_log` VALUES (7, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '139.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2025-08-28 23:40:02');
INSERT INTO `sys_login_log` VALUES (8, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '139.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2025-08-29 07:09:53');
INSERT INTO `sys_login_log` VALUES (9, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '139.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2025-08-29 07:10:10');
INSERT INTO `sys_login_log` VALUES (10, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '139.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2025-08-31 21:20:32');
INSERT INTO `sys_login_log` VALUES (11, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '139.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2025-08-31 21:58:54');
INSERT INTO `sys_login_log` VALUES (12, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '140.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2025-09-13 06:13:03');
INSERT INTO `sys_login_log` VALUES (13, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '140.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2025-09-13 06:13:19');
INSERT INTO `sys_login_log` VALUES (14, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '138.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2025-09-14 08:46:31');
INSERT INTO `sys_login_log` VALUES (15, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '140.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2025-09-14 11:42:56');
INSERT INTO `sys_login_log` VALUES (16, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '140.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2025-09-17 09:10:45');
INSERT INTO `sys_login_log` VALUES (17, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '140.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2025-09-17 09:15:46');
INSERT INTO `sys_login_log` VALUES (18, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '138.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2025-09-17 09:27:25');
INSERT INTO `sys_login_log` VALUES (19, 'jianzhu1', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '141.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2025-10-24 23:57:26');
INSERT INTO `sys_login_log` VALUES (20, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '141.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2025-10-24 23:58:03');
INSERT INTO `sys_login_log` VALUES (21, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '141.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2025-10-24 23:58:15');
INSERT INTO `sys_login_log` VALUES (22, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '143.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2025-12-27 20:25:39');
INSERT INTO `sys_login_log` VALUES (23, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '141.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-01-02 08:34:45');
INSERT INTO `sys_login_log` VALUES (24, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '143.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-01-02 08:46:18');
INSERT INTO `sys_login_log` VALUES (25, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '143.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '密码不正确', '2026-01-04 20:09:34');
INSERT INTO `sys_login_log` VALUES (26, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '143.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '密码不正确', '2026-01-04 20:09:36');
INSERT INTO `sys_login_log` VALUES (27, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '143.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '密码不正确', '2026-01-04 20:09:37');
INSERT INTO `sys_login_log` VALUES (28, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '143.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '密码不正确', '2026-01-04 20:09:38');
INSERT INTO `sys_login_log` VALUES (29, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '143.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '密码不正确', '2026-01-04 20:09:38');
INSERT INTO `sys_login_log` VALUES (30, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '143.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '密码不正确', '2026-01-04 20:09:38');
INSERT INTO `sys_login_log` VALUES (31, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '143.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-01-04 20:09:43');
INSERT INTO `sys_login_log` VALUES (32, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-23 06:00:35');
INSERT INTO `sys_login_log` VALUES (33, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-23 06:00:36');
INSERT INTO `sys_login_log` VALUES (34, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-23 06:00:37');
INSERT INTO `sys_login_log` VALUES (35, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-23 06:00:38');
INSERT INTO `sys_login_log` VALUES (36, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-23 06:00:39');
INSERT INTO `sys_login_log` VALUES (37, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-23 06:00:39');
INSERT INTO `sys_login_log` VALUES (38, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-23 06:00:39');
INSERT INTO `sys_login_log` VALUES (39, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-23 06:00:40');
INSERT INTO `sys_login_log` VALUES (40, '18613030111', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-23 06:00:40');
INSERT INTO `sys_login_log` VALUES (41, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-23 06:00:47');
INSERT INTO `sys_login_log` VALUES (42, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-23 17:10:14');
INSERT INTO `sys_login_log` VALUES (43, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 03:25:46');
INSERT INTO `sys_login_log` VALUES (44, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 03:34:09');
INSERT INTO `sys_login_log` VALUES (45, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 03:35:03');
INSERT INTO `sys_login_log` VALUES (46, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 03:36:25');
INSERT INTO `sys_login_log` VALUES (47, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 03:44:44');
INSERT INTO `sys_login_log` VALUES (48, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 03:44:51');
INSERT INTO `sys_login_log` VALUES (49, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 03:45:02');
INSERT INTO `sys_login_log` VALUES (50, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 03:45:55');
INSERT INTO `sys_login_log` VALUES (51, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 03:46:54');
INSERT INTO `sys_login_log` VALUES (52, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 03:48:42');
INSERT INTO `sys_login_log` VALUES (53, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-25 03:50:16');
INSERT INTO `sys_login_log` VALUES (54, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 03:51:15');
INSERT INTO `sys_login_log` VALUES (55, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 04:03:51');
INSERT INTO `sys_login_log` VALUES (56, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 04:04:58');
INSERT INTO `sys_login_log` VALUES (57, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 04:05:34');
INSERT INTO `sys_login_log` VALUES (58, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 04:22:16');
INSERT INTO `sys_login_log` VALUES (59, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 04:22:40');
INSERT INTO `sys_login_log` VALUES (60, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 04:25:51');
INSERT INTO `sys_login_log` VALUES (61, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 04:26:05');
INSERT INTO `sys_login_log` VALUES (62, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 04:28:07');
INSERT INTO `sys_login_log` VALUES (63, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 04:33:24');
INSERT INTO `sys_login_log` VALUES (64, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 04:35:25');
INSERT INTO `sys_login_log` VALUES (65, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 04:36:38');
INSERT INTO `sys_login_log` VALUES (66, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 04:37:07');
INSERT INTO `sys_login_log` VALUES (67, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 04:37:51');
INSERT INTO `sys_login_log` VALUES (68, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 04:50:57');
INSERT INTO `sys_login_log` VALUES (69, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 04:51:06');
INSERT INTO `sys_login_log` VALUES (70, 'adfadsf', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-24 04:53:39');
INSERT INTO `sys_login_log` VALUES (71, 'adfadsf', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-24 04:53:40');
INSERT INTO `sys_login_log` VALUES (72, 'adfadsf', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-24 04:53:40');
INSERT INTO `sys_login_log` VALUES (73, 'adfadsf', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-24 04:53:41');
INSERT INTO `sys_login_log` VALUES (74, 'adfadsf', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-24 04:53:41');
INSERT INTO `sys_login_log` VALUES (75, 'adfadsf', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-24 04:53:41');
INSERT INTO `sys_login_log` VALUES (76, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 04:53:44');
INSERT INTO `sys_login_log` VALUES (77, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 05:00:14');
INSERT INTO `sys_login_log` VALUES (78, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 05:01:57');
INSERT INTO `sys_login_log` VALUES (79, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 05:10:24');
INSERT INTO `sys_login_log` VALUES (80, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 05:10:56');
INSERT INTO `sys_login_log` VALUES (81, 'jianzhu1', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-24 05:11:11');
INSERT INTO `sys_login_log` VALUES (82, 'jianzhu1', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-24 05:11:12');
INSERT INTO `sys_login_log` VALUES (83, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 05:11:16');
INSERT INTO `sys_login_log` VALUES (84, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 05:12:54');
INSERT INTO `sys_login_log` VALUES (85, 'jianzhu1', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-24 05:13:42');
INSERT INTO `sys_login_log` VALUES (86, 'jianzhu1', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 0, '用户不存在', '2026-02-24 05:13:43');
INSERT INTO `sys_login_log` VALUES (87, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 05:13:48');
INSERT INTO `sys_login_log` VALUES (88, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 05:17:18');
INSERT INTO `sys_login_log` VALUES (89, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 05:22:19');
INSERT INTO `sys_login_log` VALUES (90, 'admin', 'todo', 'todo', 'Windows NT 10.0', 'Chrome', '145.0.0.0', 'Win64', 'x64', 'AppleWebKit/537.36', 'KHTML, like Gecko', 'Safari/537.36', 1, '登录成功', '2026-02-24 09:52:21');

-- ----------------------------
-- Table structure for sys_menu
-- ----------------------------
DROP TABLE IF EXISTS `sys_menu`;
CREATE TABLE `sys_menu`  (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '主键',
  `menu_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '菜单名称',
  `menu_type` tinyint NOT NULL DEFAULT 1 COMMENT '菜单类型(1：目录   2：菜单   3：按钮)',
  `visible` tinyint NOT NULL DEFAULT 1 COMMENT '显示状态（0:隐藏, 显示:1）',
  `status` tinyint NOT NULL DEFAULT 1 COMMENT '菜单状态(1:正常，0:禁用)',
  `sort` int NOT NULL DEFAULT 1 COMMENT '排序',
  `parent_id` bigint NOT NULL DEFAULT 0 COMMENT '父ID',
  `menu_url` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '路由路径',
  `api_url` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '接口URL',
  `menu_icon` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '菜单图标',
  `remark` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '备注',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE INDEX `menu_name`(`menu_name` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 88 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '菜单信息' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_menu
-- ----------------------------
INSERT INTO `sys_menu` VALUES (1, '首页', 1, 1, 1, 1, 0, '/home', '', 'DashboardOutlined', '首页', '2025-08-28 23:18:50', '2025-12-27 21:32:53');
INSERT INTO `sys_menu` VALUES (2, '权限管理', 1, 1, 1, 2, 0, '/system', '', 'SettingOutlined', '权限管理', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (3, '用户管理', 2, 1, 1, 1, 2, '/system/user', '', 'UserOutlined', '用户信息管理', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (4, '添加用户信息', 3, 1, 1, 1, 3, '', '/api/system/user/addUser', '', '添加用户信息', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (5, '删除用户信息', 3, 1, 1, 2, 3, '', '/api/system/user/deleteUser', '', '删除用户信息', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (6, '更新用户信息', 3, 1, 1, 3, 3, '', '/api/system/user/updateUser', '', '更新用户信息', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (7, '更新用户信息状态', 3, 1, 1, 4, 3, '', '/api/system/user/updateUserStatus', '', '更新用户信息状态', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (8, '更新用户密码', 3, 1, 1, 4, 3, '', '/api/system/user/updateUserPassword', '', '更新用户密码', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (9, '查询用户信息详情', 3, 1, 1, 5, 3, '', '/api/system/user/queryUserDetail', '', '查询用户信息详情', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (10, '查询用户信息列表', 3, 1, 1, 6, 3, '', '/api/system/user/queryUserList', '', '查询用户信息列表', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (11, '用户登录', 3, 1, 1, 7, 3, '', '/api/system/user/login', '', '用户登录', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (12, '查询用户菜单列表', 3, 1, 1, 8, 3, '', '/api/system/user/queryUserMenu', '', '查询用户菜单列表', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (13, '查询用户角色信息', 3, 1, 1, 9, 3, '', '/api/system/user/queryUserRole', '', '查询用户角色信息', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (14, '更新用户角色信息', 3, 1, 1, 10, 3, '', '/api/system/user/updateUserRole', '', '更新用户角色信息', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (15, '角色管理', 2, 1, 1, 2, 2, '/system/role', '', 'UsergroupAddOutlined', '角色信息管理', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (16, '添加角色信息', 3, 1, 1, 1, 15, '', '/api/system/role/addRole', '', '添加角色信息', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (17, '删除角色信息', 3, 1, 1, 2, 15, '', '/api/system/role/deleteRole', '', '删除角色信息', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (18, '更新角色信息', 3, 1, 1, 3, 15, '', '/api/system/role/updateRole', '', '更新角色信息', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (19, '更新角色信息状态', 3, 1, 1, 4, 15, '', '/api/system/role/updateRoleStatus', '', '更新角色信息状态', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (20, '查询角色信息详情', 3, 1, 1, 5, 15, '', '/api/system/role/queryRoleDetail', '', '查询角色信息详情', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (21, '查询角色信息列表', 3, 1, 1, 6, 15, '', '/api/system/role/queryRoleList', '', '查询角色信息列表', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (22, '查询角色菜单列表', 3, 1, 1, 7, 15, '', '/api/system/role/queryRoleMenu', '', '查询角色菜单列表', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (23, '更新角色菜单信息', 3, 1, 1, 8, 15, '', '/api/system/role/updateRoleMenu', '', '更新角色菜单信息', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (24, '查询已分配用户角色', 3, 1, 1, 8, 15, '', '/api/system/role/queryAllocatedList', '', '查询已分配用户角色', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (25, '查询未分配用户角色', 3, 1, 1, 8, 15, '', '/api/system/role/queryUnallocatedList', '', '查询未分配用户角色', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (26, '取消授权用户', 3, 1, 1, 8, 15, '', '/api/system/role/cancelAuthUser', '', '取消授权用户', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (27, '批量取消授权用户', 3, 1, 1, 8, 15, '', '/api/system/role/batchCancelAuthUser', '', '批量取消授权用户', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (28, '批量选择用户授权', 3, 1, 1, 8, 15, '', '/api/system/role/batchAuthUser', '', '批量选择用户授权', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (29, '菜单管理', 2, 1, 1, 3, 2, '/system/menu', '', 'MenuOutlined', '菜单信息管理', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (30, '添加菜单', 3, 1, 1, 1, 29, '', '/api/system/menu/addMenu', '', '添加菜单', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (31, '删除菜单', 3, 1, 1, 2, 29, '', '/api/system/menu/deleteMenu', '', '删除菜单', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (32, '更新菜单', 3, 1, 1, 3, 29, '', '/api/system/menu/updateMenu', '', '更新菜单', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (33, '更新菜单状态', 3, 1, 1, 4, 29, '', '/api/system/menu/updateMenuStatus', '', '更新菜单状态', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (34, '查询菜单详情', 3, 1, 1, 5, 29, '', '/api/system/menu/queryMenuDetail', '', '查询菜单详情', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (35, '查询菜单列表', 3, 1, 1, 6, 29, '', '/api/system/menu/queryMenuList', '', '查询菜单列表', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (36, '查询菜单树', 3, 1, 1, 6, 29, '', '/api/system/menu/queryMenuListSimple', '', '查询菜单树', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (37, '部门管理', 2, 1, 1, 4, 2, '/system/dept', '', 'ApartmentOutlined', '部门管理', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (38, '添加部门', 3, 1, 1, 1, 37, '', '/api/system/dept/addDept', '', '添加部门', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (39, '删除部门', 3, 1, 1, 2, 37, '', '/api/system/dept/deleteDept', '', '删除部门', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (40, '更新部门', 3, 1, 1, 3, 37, '', '/api/system/dept/updateDept', '', '更新部门', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (41, '更新部门状态', 3, 1, 1, 4, 37, '', '/api/system/dept/updateDeptStatus', '', '更新部门状态', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (42, '查询部门详情', 3, 1, 1, 5, 37, '', '/api/system/dept/queryDeptDetail', '', '查询部门详情', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (43, '查询部门列', 3, 1, 1, 6, 37, '', '/api/system/dept/queryDeptList', '', '查询部门列', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (44, '岗位管理', 2, 1, 1, 5, 2, '/system/post', '', 'AuditOutlined', '岗位管理', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (45, '添加岗位', 3, 1, 1, 1, 44, '', '/api/system/post/addPost', '', '添加岗位', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (46, '删除岗位', 3, 1, 1, 2, 44, '', '/api/system/post/deletePost', '', '删除岗位', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (47, '更新岗位', 3, 1, 1, 3, 44, '', '/api/system/post/updatePost', '', '更新岗位', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (48, '更新岗位状态', 3, 1, 1, 4, 44, '', '/api/system/post/updatePostStatus', '', '更新岗位状态', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (49, '查询岗位详情', 3, 1, 1, 5, 44, '', '/api/system/post/queryPostDetail', '', '查询岗位详情', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (50, '查询岗位列', 3, 1, 1, 6, 44, '', '/api/system/post/queryPostList', '', '查询岗位列', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (51, '字典类型', 2, 1, 1, 6, 2, '/system/dictType', '', 'TableOutlined', '字典类型管理', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (52, '添加字典类型', 3, 1, 1, 1, 51, '', '/api/system/dictType/addDictType', '', '添加字典类型', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (53, '删除字典类型', 3, 1, 1, 2, 51, '', '/api/system/dictType/deleteDictType', '', '删除字典类型', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (54, '更新字典类型', 3, 1, 1, 3, 51, '', '/api/system/dictType/updateDictType', '', '更新字典类型', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (55, '更新字典类型状态', 3, 1, 1, 4, 51, '', '/api/system/dictType/updateDictTypeStatus', '', '更新字典类型状态', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (56, '查询字典类型详情', 3, 1, 1, 5, 51, '', '/api/system/dictType/queryDictTypeDetail', '', '查询字典类型详情', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (57, '查询字典类型列', 3, 1, 1, 6, 51, '', '/api/system/dictType/queryDictTypeList', '', '查询字典类型列', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (58, '字典数据', 2, 0, 1, 7, 2, '/system/dictData', '', 'UngroupOutlined', '字典数据管理', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (59, '添加字典数据', 3, 1, 1, 1, 58, '', '/api/system/dictData/addDictData', '', '添加字典数据', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (60, '删除字典数据', 3, 1, 1, 2, 58, '', '/api/system/dictData/deleteDictData', '', '删除字典数据', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (61, '更新字典数据', 3, 1, 1, 3, 58, '', '/api/system/dictData/updateDictData', '', '更新字典数据', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (62, '更新字典数据状态', 3, 1, 1, 4, 58, '', '/api/system/dictData/updateDictDataStatus', '', '更新字典数据状态', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (63, '查询字典数据详情', 3, 1, 1, 5, 58, '', '/api/system/dictData/queryDictDataDetail', '', '查询字典数据详情', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (64, '查询字典数据列', 3, 1, 1, 6, 58, '', '/api/system/dictData/queryDictDataList', '', '查询字典数据列', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (65, '通知公告', 2, 1, 1, 8, 2, '/system/notice', '', 'MessageOutlined', '通知公告管理', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (66, '添加通知公告', 3, 1, 1, 1, 65, '', '/api/system/notice/addNotice', '', '添加通知公告', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (67, '删除通知公告', 3, 1, 1, 2, 65, '', '/api/system/notice/deleteNotice', '', '删除通知公告', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (68, '更新通知公告', 3, 1, 1, 3, 65, '', '/api/system/notice/updateNotice', '', '更新通知公告', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (69, '更新通知公告状态', 3, 1, 1, 4, 65, '', '/api/system/notice/updateNoticeStatus', '', '更新通知公告状态', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (70, '查询通知公告详情', 3, 1, 1, 5, 65, '', '/api/system/notice/queryNoticeDetail', '', '查询通知公告详情', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (71, '查询通知公告列', 3, 1, 1, 6, 65, '', '/api/system/notice/queryNoticeList', '', '查询通知公告列', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (72, '日志管理', 1, 1, 1, 2, 0, '/log', '', 'FilterOutlined', '日志管理', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (73, '登录日志', 2, 1, 1, 9, 72, '/log/loginLog', '', 'DeleteOutlined', '系统访问记录管理', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (74, '添加系统访问记录', 3, 1, 1, 1, 73, '', '/api/system/loginLog/addLoginLog', '', '添加系统访问记录', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (75, '删除系统访问记录', 3, 1, 1, 2, 73, '', '/api/system/loginLog/deleteLoginLog', '', '删除系统访问记录', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (76, '清空系统登录日志', 3, 1, 1, 3, 73, '', '/api/system/loginLog/cleanLoginLog', '', '清空系统登录日志', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (77, '更新系统访问记录状态', 3, 1, 1, 4, 73, '', '/api/system/loginLog/updateLoginLogStatus', '', '更新系统访问记录状态', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (78, '查询系统访问记录详情', 3, 1, 1, 5, 73, '', '/api/system/loginLog/queryLoginLogDetail', '', '查询系统访问记录详情', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (79, '查询系统访问记录列', 3, 1, 1, 6, 73, '', '/api/system/loginLog/queryLoginLogList', '', '查询系统访问记录列', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (80, '操作日志', 2, 1, 1, 10, 72, '/log/operateLog', '', 'ClearOutlined', '操作日志记录管理', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (81, '添加操作日志记录', 3, 1, 1, 1, 80, '', '/api/system/operateLog/addOperateLog', '', '添加操作日志记录', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (82, '删除操作日志记录', 3, 1, 1, 2, 80, '', '/api/system/operateLog/deleteOperateLog', '', '删除操作日志记录', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (83, '清空操作日志记录', 3, 1, 1, 3, 80, '', '/api/system/operateLog/cleanOperateLog', '', '清空操作日志记录', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (84, '更新操作日志记录状态', 3, 1, 1, 4, 80, '', '/api/system/operateLog/updateOperateLogStatus', '', '更新操作日志记录状态', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (85, '查询操作日志记录详情', 3, 1, 1, 5, 80, '', '/api/system/operateLog/queryOperateLogDetail', '', '查询操作日志记录详情', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (86, '查询操作日志记录列', 3, 1, 1, 6, 80, '', '/api/system/operateLog/queryOperateLogList', '', '查询操作日志记录列', '2025-08-28 23:18:50', '2025-08-28 23:18:50');
INSERT INTO `sys_menu` VALUES (87, '其他', 1, 1, 1, 3, 0, '/other', '', 'AudioOutlined', '其他', '2025-08-28 23:18:50', '2025-09-14 12:18:47');

-- ----------------------------
-- Table structure for sys_notice
-- ----------------------------
DROP TABLE IF EXISTS `sys_notice`;
CREATE TABLE `sys_notice`  (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '公告ID',
  `notice_title` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '公告标题',
  `notice_type` tinyint NOT NULL DEFAULT 1 COMMENT '公告类型（1:通知,2:公告）',
  `notice_content` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '公告内容',
  `status` tinyint NOT NULL DEFAULT 0 COMMENT '公告状态（0:关闭,1:正常 ）',
  `remark` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '备注',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 3 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '通知公告表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_notice
-- ----------------------------
INSERT INTO `sys_notice` VALUES (1, '测试通知1', 1, '这是一条测试通知内容', 1, '', '2025-08-28 23:18:56', '2025-08-28 23:18:56');
INSERT INTO `sys_notice` VALUES (2, '测试公告2', 2, '这是一条测试公告内容', 1, '', '2025-08-28 23:18:56', '2026-02-23 17:38:36');

-- ----------------------------
-- Table structure for sys_operate_log
-- ----------------------------
DROP TABLE IF EXISTS `sys_operate_log`;
CREATE TABLE `sys_operate_log`  (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '日志主键',
  `title` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '模块标题',
  `business_type` tinyint NULL DEFAULT 0 COMMENT '业务类型（0其它 1新增 2修改 3删除）',
  `method` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '方法名称',
  `request_method` varchar(10) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '请求方式',
  `operator_type` tinyint NULL DEFAULT 0 COMMENT '操作类别（0其它 1后台用户 2手机端用户）',
  `operate_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '操作人员',
  `dept_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '部门名称',
  `operate_url` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '请求URL',
  `operate_ip` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '主机地址',
  `operate_location` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '操作地点',
  `operate_param` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '请求参数',
  `json_result` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '返回参数',
  `status` tinyint NULL DEFAULT 0 COMMENT '操作状态(0:异常,正常)',
  `error_msg` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '错误消息',
  `operate_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '操作时间',
  `cost_time` bigint NULL DEFAULT 0 COMMENT '消耗时间',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '操作日志记录' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_operate_log
-- ----------------------------

-- ----------------------------
-- Table structure for sys_post
-- ----------------------------
DROP TABLE IF EXISTS `sys_post`;
CREATE TABLE `sys_post`  (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '岗位id',
  `post_code` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '岗位编码',
  `post_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '岗位名称',
  `sort` int NOT NULL DEFAULT 0 COMMENT '显示顺序',
  `status` tinyint NOT NULL DEFAULT 0 COMMENT '岗位状态（0：停用，1:正常）',
  `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '备注',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 5 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '岗位信息表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_post
-- ----------------------------
INSERT INTO `sys_post` VALUES (1, 'ceo', '董事长', 1, 1, '', '2025-08-28 23:19:05', '2025-08-28 23:19:05');
INSERT INTO `sys_post` VALUES (2, 'se', '项目经理', 2, 1, '', '2025-08-28 23:19:05', '2025-08-28 23:19:05');
INSERT INTO `sys_post` VALUES (3, 'hr', '人力资源', 3, 1, '', '2025-08-28 23:19:05', '2026-02-23 17:38:03');
INSERT INTO `sys_post` VALUES (4, 'user', '普通员工', 1, 1, '', '2025-08-28 23:19:05', '2025-08-28 23:19:05');

-- ----------------------------
-- Table structure for sys_role
-- ----------------------------
DROP TABLE IF EXISTS `sys_role`;
CREATE TABLE `sys_role`  (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '主键',
  `role_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '名称',
  `role_key` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '角色权限字符串',
  `data_scope` tinyint NOT NULL DEFAULT 1 COMMENT '数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）',
  `status` tinyint NOT NULL DEFAULT 1 COMMENT '状态(1:正常，0:禁用)',
  `remark` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '备注',
  `del_flag` tinyint NOT NULL DEFAULT 1 COMMENT '删除标志（0代表删除 1代表存在）',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE INDEX `role_name`(`role_name` ASC) USING BTREE,
  INDEX `name_status_index`(`role_name` ASC, `status` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 6 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '角色信息' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_role
-- ----------------------------
INSERT INTO `sys_role` VALUES (1, '超级管理员', 'admin', 1, 1, '全部权限', 1, '2025-08-28 23:19:10', '2025-08-28 23:19:10');
INSERT INTO `sys_role` VALUES (2, '演示角色', 'query', 1, 1, '仅有查看功能', 1, '2025-08-28 23:19:10', '2025-12-27 21:18:04');
INSERT INTO `sys_role` VALUES (3, '121', 'dev', 1, 0, '121211', 1, '2025-08-28 23:19:10', '2025-08-28 23:19:10');
INSERT INTO `sys_role` VALUES (4, '123', '1312', 1, 1, '1321', 1, '2025-09-14 08:47:13', '2025-09-14 08:47:13');
INSERT INTO `sys_role` VALUES (5, '12343343', '123123', 1, 1, '12312', 1, '2025-09-17 09:28:07', '2025-09-17 09:28:06');

-- ----------------------------
-- Table structure for sys_role_dept
-- ----------------------------
DROP TABLE IF EXISTS `sys_role_dept`;
CREATE TABLE `sys_role_dept`  (
  `role_id` bigint NOT NULL COMMENT '角色id',
  `dept_id` bigint NOT NULL COMMENT '部门id',
  PRIMARY KEY (`role_id`, `dept_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '角色和部门关联表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_role_dept
-- ----------------------------

-- ----------------------------
-- Table structure for sys_role_menu
-- ----------------------------
DROP TABLE IF EXISTS `sys_role_menu`;
CREATE TABLE `sys_role_menu`  (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '主键',
  `role_id` bigint NOT NULL COMMENT '角色ID',
  `menu_id` bigint NOT NULL COMMENT '菜单ID',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 203 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '菜单角色关联表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_role_menu
-- ----------------------------
INSERT INTO `sys_role_menu` VALUES (27, 3, 1, '2025-09-14 00:54:21');
INSERT INTO `sys_role_menu` VALUES (28, 3, 87, '2025-09-14 00:54:21');
INSERT INTO `sys_role_menu` VALUES (116, 2, 1, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (117, 2, 87, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (118, 2, 2, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (119, 2, 3, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (120, 2, 15, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (121, 2, 29, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (122, 2, 37, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (123, 2, 44, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (124, 2, 51, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (125, 2, 58, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (126, 2, 65, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (127, 2, 4, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (128, 2, 5, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (129, 2, 6, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (130, 2, 7, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (131, 2, 8, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (132, 2, 9, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (133, 2, 10, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (134, 2, 11, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (135, 2, 12, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (136, 2, 13, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (137, 2, 14, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (138, 2, 16, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (139, 2, 17, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (140, 2, 18, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (141, 2, 19, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (142, 2, 20, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (143, 2, 21, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (144, 2, 22, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (145, 2, 23, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (146, 2, 24, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (147, 2, 25, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (148, 2, 26, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (149, 2, 27, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (150, 2, 28, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (151, 2, 30, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (152, 2, 31, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (153, 2, 32, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (154, 2, 33, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (155, 2, 34, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (156, 2, 35, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (157, 2, 36, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (158, 2, 38, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (159, 2, 39, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (160, 2, 40, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (161, 2, 41, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (162, 2, 42, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (163, 2, 43, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (164, 2, 45, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (165, 2, 46, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (166, 2, 47, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (167, 2, 48, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (168, 2, 49, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (169, 2, 50, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (170, 2, 52, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (171, 2, 53, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (172, 2, 54, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (173, 2, 55, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (174, 2, 56, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (175, 2, 57, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (176, 2, 59, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (177, 2, 60, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (178, 2, 61, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (179, 2, 62, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (180, 2, 63, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (181, 2, 64, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (182, 2, 66, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (183, 2, 67, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (184, 2, 68, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (185, 2, 69, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (186, 2, 70, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (187, 2, 71, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (188, 2, 72, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (189, 2, 73, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (190, 2, 80, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (191, 2, 74, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (192, 2, 75, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (193, 2, 76, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (194, 2, 77, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (195, 2, 78, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (196, 2, 79, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (197, 2, 81, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (198, 2, 82, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (199, 2, 83, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (200, 2, 84, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (201, 2, 85, '2025-12-27 21:18:10');
INSERT INTO `sys_role_menu` VALUES (202, 2, 86, '2025-12-27 21:18:10');

-- ----------------------------
-- Table structure for sys_user
-- ----------------------------
DROP TABLE IF EXISTS `sys_user`;
CREATE TABLE `sys_user`  (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '主键',
  `mobile` char(11) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '手机号码',
  `user_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '用户账号',
  `nick_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '用户昵称',
  `user_type` varchar(2) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '00' COMMENT '用户类型（00系统用户）',
  `avatar` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '头像路径',
  `email` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '用户邮箱',
  `password` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '密码',
  `status` tinyint NOT NULL DEFAULT 1 COMMENT '状态(1:正常，0:禁用)',
  `dept_id` bigint NOT NULL DEFAULT 1 COMMENT '部门ID',
  `login_ip` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '最后登录IP',
  `login_date` datetime NULL DEFAULT NULL COMMENT '最后登录时间',
  `login_browser` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '浏览器类型',
  `login_os` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '操作系统',
  `pwd_update_date` datetime NULL DEFAULT NULL COMMENT '密码最后更新时间',
  `remark` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '备注',
  `del_flag` tinyint NOT NULL DEFAULT 1 COMMENT '删除标志（0代表删除 1代表存在）',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE INDEX `AK_phone`(`mobile` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 3 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '用户信息' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_user
-- ----------------------------
INSERT INTO `sys_user` VALUES (1, 'admin', 'admin', 'admin', '00', '', 'xx@qq.com', '123456', 1, 1, '', '2026-02-24 09:52:22', 'Chrome', 'Win64', NULL, '超级管理员', 1, '2025-08-28 23:19:23', '2026-01-02 08:46:02');
INSERT INTO `sys_user` VALUES (2, 'test', 'test', 'test', '01', 'https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png', '123@qq.com', '', 1, 1, '', NULL, '', '', NULL, '演示权限', 1, '2025-08-28 23:19:23', '2026-01-02 08:46:04');

-- ----------------------------
-- Table structure for sys_user_post
-- ----------------------------
DROP TABLE IF EXISTS `sys_user_post`;
CREATE TABLE `sys_user_post`  (
  `user_id` bigint NOT NULL COMMENT '用户id',
  `post_id` bigint NOT NULL COMMENT '岗位id',
  PRIMARY KEY (`user_id`, `post_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '用户与岗位关联表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_user_post
-- ----------------------------
INSERT INTO `sys_user_post` VALUES (1, 1);
INSERT INTO `sys_user_post` VALUES (2, 2);

-- ----------------------------
-- Table structure for sys_user_role
-- ----------------------------
DROP TABLE IF EXISTS `sys_user_role`;
CREATE TABLE `sys_user_role`  (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '主键',
  `user_id` bigint NOT NULL DEFAULT 0 COMMENT '用户ID',
  `role_id` bigint NOT NULL COMMENT '角色ID',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 4 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '角色用户关联表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_user_role
-- ----------------------------
INSERT INTO `sys_user_role` VALUES (1, 1, 1, '2025-08-28 23:19:32');
INSERT INTO `sys_user_role` VALUES (2, 1, 3, '2025-09-14 00:53:47');

SET FOREIGN_KEY_CHECKS = 1;
