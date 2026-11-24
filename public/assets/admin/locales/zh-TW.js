(() => {
  const translations = {
    "giftCard": {
      "title": "\u79AE\u54C1\u5361\u7BA1\u7406",
      "description": "\u5728\u9019\u88E1\u53EF\u4EE5\u7BA1\u7406\u79AE\u54C1\u5361\u6A21\u677F\u3001\u514C\u63DB\u78BC\u548C\u4F7F\u7528\u8A18\u9304\u7B49\u529F\u80FD\u3002",
      "tabs": {
        "templates": "\u6A21\u677F\u7BA1\u7406",
        "codes": "\u514C\u63DB\u78BC\u7BA1\u7406",
        "usages": "\u4F7F\u7528\u8A18\u9304",
        "statistics": "\u7D71\u8A08\u6578\u64DA"
      },
      "template": {
        "title": "\u6A21\u677F\u7BA1\u7406",
        "description": "\u7BA1\u7406\u79AE\u54C1\u5361\u6A21\u677F\uFF0C\u5305\u62EC\u5275\u5EFA\u3001\u7DE8\u8F2F\u548C\u522A\u9664\u6A21\u677F\u3002",
        "table": {
          "title": "\u6A21\u677F\u5217\u8868",
          "columns": {
            "id": "ID",
            "name": "\u540D\u7A31",
            "type": "\u985E\u578B",
            "status": "\u72C0\u614B",
            "sort": "\u6392\u5E8F",
            "rewards": "\u734E\u52F5\u5167\u5BB9",
            "created_at": "\u5275\u5EFA\u6642\u9593",
            "actions": "\u64CD\u4F5C",
            "no_rewards": "\u7121\u734E\u52F5"
          }
        },
        "form": {
          "add": "\u6DFB\u52A0\u6A21\u677F",
          "edit": "\u7DE8\u8F2F\u6A21\u677F",
          "name": {
            "label": "\u6A21\u677F\u540D\u7A31",
            "placeholder": "\u8ACB\u8F38\u5165\u6A21\u677F\u540D\u7A31",
            "required": "\u8ACB\u8F38\u5165\u6A21\u677F\u540D\u7A31"
          },
          "sort": {
            "label": "\u6392\u5E8F",
            "placeholder": "\u6578\u5B57\u8D8A\u5C0F\u8D8A\u9760\u524D"
          },
          "type": {
            "label": "\u985E\u578B",
            "placeholder": "\u8ACB\u9078\u64C7\u79AE\u54C1\u5361\u985E\u578B"
          },
          "description": {
            "label": "\u63CF\u8FF0",
            "placeholder": "\u8ACB\u8F38\u5165\u79AE\u54C1\u5361\u63CF\u8FF0"
          },
          "status": {
            "label": "\u72C0\u614B",
            "description": "\u7981\u7528\u5F8C\uFF0C\u6B64\u6A21\u677F\u5C07\u7121\u6CD5\u751F\u6210\u6216\u514C\u63DB\u65B0\u7684\u79AE\u54C1\u5361\u3002"
          },
          "display": {
            "title": "\u986F\u793A\u6548\u679C",
            "theme_color": {
              "label": "\u4E3B\u984C\u984F\u8272"
            },
            "icon": {
              "label": "\u5716\u6A19",
              "placeholder": "\u8ACB\u8F38\u5165\u5716\u6A19\u7684URL"
            },
            "background_image": {
              "label": "\u80CC\u666F\u5716\u7247",
              "placeholder": "\u8ACB\u8F38\u5165\u80CC\u666F\u5716\u7247\u7684URL"
            }
          },
          "conditions": {
            "title": "\u4F7F\u7528\u689D\u4EF6",
            "new_user_max_days": {
              "label": "\u65B0\u7528\u6236\u8A3B\u518A\u5929\u6578\u9650\u5236",
              "placeholder": "\u4F8B\u5982: 7 (\u50C5\u9650\u8A3B\u518A7\u5929\u5167\u7684\u7528\u6236)"
            },
            "new_user_only": {
              "label": "\u50C5\u9650\u65B0\u7528\u6236",
              "hint": "\u9700\u8A3B\u518A\u5929\u6578 \u2264 {{days}}"
            },
            "paid_user_only": {
              "label": "\u50C5\u9650\u4ED8\u8CBB\u7528\u6236"
            },
            "require_invite": {
              "label": "\u9700\u8981\u9080\u8ACB\u95DC\u4FC2"
            },
            "allowed_plans": {
              "label": "\u5141\u8A31\u7684\u5957\u9910",
              "placeholder": "\u9078\u64C7\u5141\u8A31\u514C\u63DB\u7684\u5957\u9910 (\u7559\u7A7A\u5247\u4E0D\u9650\u5236)"
            },
            "disallowed_plans": {
              "label": "\u7981\u6B62\u7684\u5957\u9910",
              "placeholder": "\u9078\u64C7\u7981\u6B62\u514C\u63DB\u7684\u5957\u9910 (\u7559\u7A7A\u5247\u4E0D\u9650\u5236)"
            }
          },
          "limits": {
            "title": "\u4F7F\u7528\u9650\u5236",
            "max_use_per_user": {
              "label": "\u55AE\u7528\u6236\u6700\u5927\u4F7F\u7528\u6B21\u6578",
              "placeholder": "\u7559\u7A7A\u5247\u4E0D\u9650\u5236"
            },
            "cooldown_hours": {
              "label": "\u540C\u985E\u5361\u51B7\u537B\u6642\u9593(\u5C0F\u6642)",
              "placeholder": "\u7559\u7A7A\u5247\u4E0D\u9650\u5236"
            }
          },
          "rewards": {
            "title": "\u734E\u52F5\u5167\u5BB9",
            "invite_reward_rate": {
              "label": "\u9080\u8ACB\u4EBA\u734E\u52F5\u6BD4\u4F8B",
              "placeholder": "\u4F8B\u5982: 0.2 (\u4EE3\u886820%)",
              "description": "\u4F7F\u7528\u8005\u6709\u9080\u8ACB\u4EBA\u6642\uFF0C\u7D66\u9080\u8ACB\u4EBA\u7684\u734E\u52F5 = \u9918\u984D\u734E\u52F5 * \u6B64\u6BD4\u4F8B"
            },
            "balance": {
              "label": "\u734E\u52F5\u9918\u984D (\u5143)",
              "short_label": "\u9918\u984D",
              "placeholder": "\u8ACB\u8F38\u5165\u734E\u52F5\u7684\u91D1\u984D(\u5143)"
            },
            "transfer_enable": {
              "label": "\u734E\u52F5\u6D41\u91CF (GB)",
              "short_label": "\u6D41\u91CF",
              "placeholder": "\u8ACB\u8F38\u5165\u734E\u52F5\u7684\u6D41\u91CF(GB)"
            },
            "expire_days": {
              "label": "\u5EF6\u9577\u6709\u6548\u671F (\u5929)",
              "short_label": "\u6709\u6548\u671F",
              "placeholder": "\u8ACB\u8F38\u5165\u5EF6\u9577\u7684\u5929\u6578"
            },
            "transfer": {
              "label": "\u734E\u52F5\u6D41\u91CF (\u5B57\u7BC0)",
              "placeholder": "\u8ACB\u8F38\u5165\u734E\u52F5\u7684\u6D41\u91CF(\u5B57\u7BC0)"
            },
            "days": {
              "label": "\u5EF6\u9577\u6709\u6548\u671F (\u5929)",
              "placeholder": "\u8ACB\u8F38\u5165\u5EF6\u9577\u7684\u5929\u6578"
            },
            "device_limit": {
              "label": "\u589E\u52A0\u8A2D\u5099\u6578",
              "short_label": "\u8A2D\u5099\u6578",
              "placeholder": "\u8ACB\u8F38\u5165\u589E\u52A0\u7684\u8A2D\u5099\u6578\u91CF"
            },
            "reset_package": {
              "label": "\u91CD\u7F6E\u7576\u6708\u6D41\u91CF",
              "description": "\u958B\u555F\u5F8C\uFF0C\u514C\u63DB\u6642\u6703\u5C07\u7528\u6236\u7576\u524D\u5957\u9910\u7684\u5DF2\u7528\u6D41\u91CF\u6E05\u96F6\u3002"
            },
            "reset_count": {
              "description": "\u8A72\u985E\u578B\u5361\u5C07\u91CD\u7F6E\u7528\u6236\u7576\u6708\u7684\u6D41\u91CF\u4F7F\u7528\u3002"
            },
            "task_card": {
              "description": "\u4EFB\u52D9\u79AE\u54C1\u5361\u7684\u5177\u9AD4\u734E\u52F5\u5C07\u5728\u4EFB\u52D9\u7CFB\u7D71\u4E2D\u914D\u7F6E\u3002"
            },
            "plan_id": {
              "label": "\u6307\u5B9A\u5957\u9910",
              "short_label": "\u5957\u9910",
              "placeholder": "\u8ACB\u9078\u64C7\u4E00\u500B\u5957\u9910"
            },
            "plan_validity_days": {
              "label": "\u5957\u9910\u6709\u6548\u671F (\u5929)",
              "short_label": "\u5957\u9910\u6709\u6548\u671F",
              "placeholder": "\u7559\u7A7A\u5247\u4F7F\u7528\u5957\u9910\u9ED8\u8A8D\u6709\u6548\u671F"
            },
            "random_rewards": {
              "label": "\u96A8\u6A5F\u734E\u52F5\u6C60",
              "add": "\u6DFB\u52A0\u96A8\u6A5F\u734E\u52F5\u9805",
              "weight": "\u6B0A\u91CD"
            }
          },
          "special_config": {
            "title": "\u7279\u6B8A\u914D\u7F6E",
            "start_time": {
              "label": "\u6D3B\u52D5\u958B\u59CB\u6642\u9593",
              "placeholder": "\u8ACB\u9078\u64C7\u958B\u59CB\u65E5\u671F"
            },
            "end_time": {
              "label": "\u6D3B\u52D5\u7D50\u675F\u6642\u9593",
              "placeholder": "\u8ACB\u9078\u64C7\u7D50\u675F\u65E5\u671F"
            },
            "festival_bonus": {
              "label": "\u7BC0\u65E5\u734E\u52F5\u4E58\u6578",
              "placeholder": "\u4F8B\u5982: 1.5 (\u4EE3\u88681.5\u500D)"
            }
          },
          "submit": {
            "saving": "\u4FDD\u5B58\u4E2D...",
            "save": "\u4FDD\u5B58"
          }
        },
        "actions": {
          "edit": "\u7DE8\u8F2F",
          "delete": "\u522A\u9664",
          "deleteConfirm": {
            "title": "\u78BA\u8A8D\u522A\u9664",
            "description": "\u6B64\u64CD\u4F5C\u5C07\u6C38\u4E45\u522A\u9664\u8A72\u6A21\u677F\uFF0C\u78BA\u5B9A\u8981\u7E7C\u7E8C\u55CE\uFF1F",
            "confirmText": "\u522A\u9664"
          }
        }
      },
      "code": {
        "title": "\u514C\u63DB\u78BC\u7BA1\u7406",
        "form": {
          "generate": "\u751F\u6210\u514C\u63DB\u78BC",
          "template_id": {
            "label": "\u9078\u64C7\u6A21\u677F",
            "placeholder": "\u8ACB\u9078\u64C7\u4E00\u500B\u6A21\u677F\u4F86\u751F\u6210\u514C\u63DB\u78BC"
          },
          "count": {
            "label": "\u751F\u6210\u6578\u91CF"
          },
          "prefix": {
            "label": "\u81EA\u5B9A\u7FA9\u524D\u7DB4 (\u53EF\u9078)"
          },
          "expires_hours": {
            "label": "\u6709\u6548\u671F (\u5C0F\u6642)"
          },
          "max_usage": {
            "label": "\u6700\u5927\u4F7F\u7528\u6B21\u6578"
          },
          "download_csv": "\u5C0E\u51FACSV",
          "submit": {
            "generating": "\u751F\u6210\u4E2D...",
            "generate": "\u7ACB\u5373\u751F\u6210"
          }
        },
        "description": "\u7BA1\u7406\u79AE\u54C1\u5361\u514C\u63DB\u78BC\uFF0C\u5305\u62EC\u751F\u6210\u3001\u67E5\u770B\u548C\u5C0E\u51FA\u514C\u63DB\u78BC\u3002",
        "generate": {
          "title": "\u751F\u6210\u514C\u63DB\u78BC",
          "template": "\u9078\u64C7\u6A21\u677F",
          "count": "\u751F\u6210\u6578\u91CF",
          "prefix": "\u81EA\u5B9A\u7FA9\u524D\u7DB4",
          "expires_hours": "\u6709\u6548\u671F (\u5C0F\u6642)",
          "max_usage": "\u6700\u5927\u4F7F\u7528\u6B21\u6578",
          "submit": "\u751F\u6210"
        },
        "table": {
          "title": "\u514C\u63DB\u78BC\u5217\u8868",
          "columns": {
            "id": "ID",
            "code": "\u514C\u63DB\u78BC",
            "template_name": "\u6A21\u677F\u540D\u7A31",
            "status": "\u72C0\u614B",
            "expires_at": "\u904E\u671F\u6642\u9593",
            "usage_count": "\u5DF2\u7528\u6B21\u6578",
            "max_usage": "\u53EF\u7528\u6B21\u6578",
            "created_at": "\u5275\u5EFA\u6642\u9593"
          }
        },
        "actions": {
          "enable": "\u555F\u7528",
          "disable": "\u7981\u7528",
          "export": "\u5C0E\u51FA",
          "exportConfirm": {
            "title": "\u78BA\u8A8D\u5C0E\u51FA",
            "description": "\u5C07\u5C0E\u51FA\u9078\u5B9A\u6279\u6B21\u7684\u6240\u6709\u514C\u63DB\u78BC\u70BA\u6587\u672C\u6587\u4EF6\u3002\u78BA\u5B9A\u8981\u7E7C\u7E8C\u55CE\uFF1F",
            "confirmText": "\u5C0E\u51FA"
          }
        },
        "status": {
          "0": "\u672A\u4F7F\u7528",
          "1": "\u5DF2\u4F7F\u7528",
          "2": "\u5DF2\u904E\u671F",
          "3": "\u5DF2\u7981\u7528"
        },
        "edit": {
          "title": "\u7DE8\u8F2F\u79AE\u54C1\u78BC",
          "code": "\u79AE\u54C1\u78BC",
          "template": "\u6A21\u677F",
          "templatePlaceholder": "\u8ACB\u9078\u64C7\u6A21\u677F",
          "maxUsage": "\u6700\u5927\u4F7F\u7528\u6B21\u6578",
          "status": "\u72C0\u614B",
          "expiresAt": "\u904E\u671F\u6642\u9593"
        },
        "messages": {
          "enabled": "\u5DF2\u555F\u7528",
          "disabled": "\u5DF2\u7981\u7528",
          "exportSuccess": "\u5C0E\u51FA\u6210\u529F",
          "deleteConfirmTitle": "\u78BA\u8A8D\u522A\u9664",
          "deleteConfirmDescription": "\u78BA\u5B9A\u8981\u522A\u9664\u79AE\u54C1\u78BC {{code}} \u55CE\uFF1F\u6B64\u64CD\u4F5C\u4E0D\u53EF\u64A4\u92B7\u3002",
          "deleteSuccess": "\u522A\u9664\u6210\u529F",
          "selectTemplate": "\u8ACB\u9078\u64C7\u6A21\u677F",
          "updateSuccess": "\u66F4\u65B0\u6210\u529F"
        }
      },
      "usage": {
        "title": "\u4F7F\u7528\u8A18\u9304",
        "description": "\u67E5\u770B\u79AE\u54C1\u5361\u7684\u4F7F\u7528\u8A18\u9304\u548C\u8A73\u7D30\u4FE1\u606F\u3002",
        "table": {
          "columns": {
            "id": "ID",
            "code": "\u514C\u63DB\u78BC",
            "template_name": "\u6A21\u677F\u540D\u7A31",
            "user_email": "\u7528\u6236\u90F5\u7BB1",
            "rewards_given": "\u7372\u5F97\u734E\u52F5",
            "invite_rewards": "\u9080\u8ACB\u734E\u52F5",
            "multiplier_applied": "\u500D\u6578\u52A0\u6210",
            "ip_address": "IP\u5730\u5740",
            "created_at": "\u4F7F\u7528\u6642\u9593",
            "actions": "\u64CD\u4F5C"
          }
        },
        "actions": {
          "view": "\u67E5\u770B\u8A73\u60C5"
        }
      },
      "statistics": {
        "title": "\u7D71\u8A08\u6578\u64DA",
        "description": "\u67E5\u770B\u79AE\u54C1\u5361\u7684\u7D71\u8A08\u6578\u64DA\u548C\u4F7F\u7528\u60C5\u6CC1\u5206\u6790\u3002",
        "total": {
          "title": "\u7E3D\u9AD4\u7D71\u8A08",
          "templates_count": "\u6A21\u677F\u7E3D\u6578",
          "active_templates_count": "\u6D3B\u8E8D\u6A21\u677F\u6578",
          "codes_count": "\u514C\u63DB\u78BC\u7E3D\u6578",
          "used_codes_count": "\u5DF2\u4F7F\u7528\u514C\u63DB\u78BC",
          "usages_count": "\u4F7F\u7528\u8A18\u9304\u6578"
        },
        "daily": {
          "title": "\u6BCF\u65E5\u4F7F\u7528\u91CF",
          "chart": "\u4F7F\u7528\u91CF\u8DA8\u52E2\u5716"
        },
        "type": {
          "title": "\u985E\u578B\u7D71\u8A08",
          "chart": "\u985E\u578B\u5206\u4F48\u5716"
        },
        "dateRange": {
          "label": "\u65E5\u671F\u7BC4\u570D",
          "start": "\u958B\u59CB\u65E5\u671F",
          "end": "\u7D50\u675F\u65E5\u671F"
        }
      },
      "types": {
        "1": "\u901A\u7528\u79AE\u54C1\u5361",
        "2": "\u5957\u9910\u79AE\u54C1\u5361",
        "3": "\u76F2\u76D2\u79AE\u54C1\u5361",
        "4": "\u4EFB\u52D9\u79AE\u54C1\u5361"
      },
      "common": {
        "search": "\u641C\u7D22\u79AE\u54C1\u5361...",
        "reset": "\u91CD\u7F6E",
        "filter": "\u7BE9\u9078",
        "export": "\u5C0E\u51FA",
        "refresh": "\u5237\u65B0",
        "back": "\u8FD4\u56DE",
        "close": "\u95DC\u9589",
        "confirm": "\u78BA\u8A8D",
        "cancel": "\u53D6\u6D88",
        "enabled": "\u5DF2\u555F\u7528",
        "disabled": "\u5DF2\u7981\u7528",
        "loading": "\u52A0\u8F09\u4E2D...",
        "noData": "\u66AB\u7121\u6578\u64DA",
        "success": "\u64CD\u4F5C\u6210\u529F",
        "error": "\u64CD\u4F5C\u5931\u6557"
      },
      "messages": {
        "formInvalid": "\u8ACB\u6AA2\u67E5\u8868\u55AE\u8F38\u5165\u662F\u5426\u6B63\u78BA",
        "templateCreated": "\u6A21\u677F\u5275\u5EFA\u6210\u529F",
        "templateUpdated": "\u6A21\u677F\u66F4\u65B0\u6210\u529F",
        "templateDeleted": "\u6A21\u677F\u522A\u9664\u6210\u529F",
        "codeGenerated": "\u514C\u63DB\u78BC\u751F\u6210\u6210\u529F",
        "generateCodeFailed": "\u514C\u63DB\u78BC\u751F\u6210\u5931\u6557",
        "codeStatusUpdated": "\u514C\u63DB\u78BC\u72C0\u614B\u66F4\u65B0\u6210\u529F",
        "updateCodeStatusFailed": "\u514C\u63DB\u78BC\u72C0\u614B\u66F4\u65B0\u5931\u6557",
        "codesExported": "\u514C\u63DB\u78BC\u5C0E\u51FA\u6210\u529F",
        "createTemplateFailed": "\u5275\u5EFA\u6A21\u677F\u5931\u6557",
        "updateTemplateFailed": "\u66F4\u65B0\u6A21\u677F\u5931\u6557",
        "deleteTemplateFailed": "\u522A\u9664\u6A21\u677F\u5931\u6557",
        "loadDataFailed": "\u52A0\u8F09\u6578\u64DA\u5931\u6557",
        "codesGenerated": "\u514C\u63DB\u78BC\u751F\u6210\u6210\u529F"
      }
    },
    "user": {
      "manage": {
        "title": "\u7528\u6236\u7BA1\u7406",
        "description": "\u5728\u9019\u88E1\u53EF\u4EE5\u7BA1\u7406\u7528\u6236\uFF0C\u5305\u62EC\u589E\u52A0\u3001\u522A\u9664\u3001\u7DE8\u8F2F\u3001\u67E5\u8A62\u7B49\u64CD\u4F5C\u3002"
      },
      "columns": {
        "is_admin": "\u7BA1\u7406\u54E1",
        "is_staff": "\u54E1\u5DE5",
        "id": "ID",
        "email": "\u90F5\u7BB1",
        "online_count": "\u5728\u7DDA\u8A2D\u5099",
        "status": "\u72C0\u614B",
        "subscription": "\u8A02\u95B1",
        "group": "\u6B0A\u9650\u7D44",
        "used_traffic": "\u5DF2\u7528\u6D41\u91CF",
        "total_traffic": "\u7E3D\u6D41\u91CF",
        "expire_time": "\u5230\u671F\u6642\u9593",
        "balance": "\u9918\u984D",
        "commission": "\u4F63\u91D1",
        "register_time": "\u8A3B\u518A\u6642\u9593",
        "invitee_email": "\u9080\u8ACB\u7528\u6236",
        "actions": "\u64CD\u4F5C",
        "next_reset_at": "\u4E0B\u6B21\u91CD\u7F6E\u6642\u9593",
        "device_limit": {
          "unlimited": "\u7121\u8A2D\u5099\u6578\u9650\u5236",
          "limited": "\u6700\u591A\u53EF\u540C\u6642\u5728\u7DDA {{count}} \u81FA\u8A2D\u5099"
        },
        "status_text": {
          "normal": "\u6B63\u5E38",
          "banned": "\u5DF2\u5C01\u7981"
        },
        "online_status": {
          "online": "\u7576\u524D\u5728\u7DDA",
          "never": "\u5F9E\u672A\u5728\u7DDA",
          "last_online": "\u6700\u5F8C\u5728\u7DDA\u6642\u9593: {{time}}",
          "offline_duration": {
            "days": "\u96E2\u7DDA\u6642\u9577: {{count}}\u5929",
            "hours": "\u96E2\u7DDA\u6642\u9577: {{count}}\u5C0F\u6642",
            "minutes": "\u96E2\u7DDA\u6642\u9577: {{count}}\u5206\u9418",
            "seconds": "\u96E2\u7DDA\u6642\u9577: {{count}}\u79D2"
          }
        },
        "expire_status": {
          "permanent": "\u9577\u671F\u6709\u6548",
          "expired": "\u5DF2\u904E\u671F {{days}} \u5929",
          "remaining": "\u5269\u9918 {{days}} \u5929"
        },
        "copy_email": "\u8907\u88FD\u90F5\u7BB1",
        "actions_menu": {
          "edit": "\u7DE8\u8F2F",
          "view_details": "\u67E5\u770B\u8A73\u60C5",
          "assign_order": "\u5206\u914D\u8A02\u55AE",
          "copy_url": "\u8907\u88FD\u8A02\u95B1URL",
          "reset_secret": "\u91CD\u7F6EUUID\u53CA\u8A02\u95B1URL",
          "reset_traffic": "\u91CD\u7F6E\u6D41\u91CF",
          "orders": "TA\u7684\u8A02\u55AE",
          "invites": "TA\u7684\u9080\u8ACB",
          "traffic_records": "TA\u7684\u6D41\u91CF\u8A18\u9304",
          "login_history": "\u767B\u9304\u6B77\u53F2",
          "delete": "\u522A\u9664",
          "delete_confirm_title": "\u78BA\u8A8D\u522A\u9664\u7528\u6236",
          "delete_confirm_description": "\u6B64\u64CD\u4F5C\u5C07\u6C38\u4E45\u522A\u9664\u7528\u6236 {{email}} \u53CA\u5176\u6240\u6709\u76F8\u95DC\u6578\u64DA\uFF0C\u5305\u62EC\u8A02\u55AE\u3001\u512A\u60E0\u78BC\u3001\u6D41\u91CF\u8A18\u9304\u3001\u5DE5\u55AE\u8A18\u9304\u7B49\u4FE1\u606F\u3002\u522A\u9664\u5F8C\u7121\u6CD5\u6062\u5FA9\uFF0C\u662F\u5426\u7E7C\u7E8C\uFF1F"
        }
      },
      "filter": {
        "selected": "\u5DF2\u9078\u64C7 {{count}} \u9805",
        "clear_selection": "\u53D6\u6D88\u9078\u64C7",
        "no_results": "\u672A\u627E\u5230\u7D50\u679C",
        "clear": "\u6E05\u9664\u7BE9\u9078",
        "search_placeholder": "\u641C\u7D22...",
        "email_search": "\u641C\u7D22\u7528\u6236\u90F5\u7BB1...",
        "advanced": "\u9AD8\u7D1A\u7BE9\u9078",
        "reset": "\u91CD\u7F6E\u7BE9\u9078",
        "sheet": {
          "title": "\u9AD8\u7D1A\u7BE9\u9078",
          "description": "\u6DFB\u52A0\u4E00\u500B\u6216\u591A\u500B\u7BE9\u9078\u689D\u4EF6\u4F86\u7CBE\u78BA\u67E5\u627E\u7528\u6236",
          "conditions": "\u7BE9\u9078\u689D\u4EF6",
          "add": "\u6DFB\u52A0\u689D\u4EF6",
          "condition": "\u689D\u4EF6 {{number}}",
          "field": "\u9078\u64C7\u5B57\u6BB5",
          "operator": "\u9078\u64C7\u64CD\u4F5C\u7B26",
          "value": "\u8F38\u5165\u503C",
          "value_number": "\u8F38\u5165\u6578\u503C({{unit}})",
          "reset": "\u91CD\u7F6E",
          "apply": "\u61C9\u7528\u7BE9\u9078"
        },
        "fields": {
          "email": "\u90F5\u7BB1",
          "phone": "\u624B\u6A5F\u865F",
          "id": "\u7528\u6236ID",
          "plan_id": "\u8A02\u95B1",
          "transfer_enable": "\u6D41\u91CF",
          "total_used": "\u5DF2\u7528\u6D41\u91CF",
          "online_count": "\u5728\u7DDA\u8A2D\u5099",
          "expired_at": "\u5230\u671F\u6642\u9593",
          "uuid": "UUID",
          "token": "Token",
          "banned": "\u8CEC\u865F\u72C0\u614B",
          "remark": "\u5099\u8A3B",
          "inviter_email": "\u9080\u8ACB\u4EBA\u90F5\u7BB1",
          "invite_user_id": "\u9080\u8ACB\u4EBAID",
          "is_admin": "\u7BA1\u7406\u54E1",
          "is_staff": "\u54E1\u5DE5"
        },
        "operators": {
          "contains": "\u5305\u542B",
          "eq": "\u7B49\u65BC",
          "gt": "\u5927\u65BC",
          "lt": "\u5C0F\u65BC"
        },
        "status": {
          "normal": "\u6B63\u5E38",
          "banned": "\u7981\u7528"
        },
        "boolean": {
          "true": "\u662F",
          "false": "\u5426"
        }
      },
      "generate": {
        "button": "\u5275\u5EFA\u7528\u6236",
        "title": "\u5275\u5EFA\u7528\u6236",
        "description_single": "\u5275\u5EFA\u55AE\u500B\u6307\u5B9A\u90F5\u7BB1\u8CEC\u865F",
        "description_batch": "\u6279\u91CF\u751F\u6210\u96A8\u6A5F\u90F5\u7BB1\u8CEC\u865F",
        "form": {
          "email": "\u90F5\u7BB1",
          "email_prefix": "\u5E33\u865F(\u6279\u91CF\u751F\u6210\u8ACB\u7559\u7A7A)",
          "email_prefix_placeholder": "\u7559\u7A7A\u5247\u6279\u91CF\u96A8\u6A5F",
          "email_domain": "\u57DF",
          "password": "\u5BC6\u78BC",
          "password_placeholder": "\u7559\u7A7A\u5247\u5BC6\u78BC\u8207\u90F5\u4EF6\u76F8\u540C",
          "expire_time": "\u5230\u671F\u6642\u9593",
          "expire_time_placeholder": "\u8ACB\u9078\u64C7\u7528\u6236\u5230\u671F\u65E5\u671F\uFF0C\u7559\u7A7A\u70BA\u9577\u671F\u6709\u6548",
          "permanent": "\u9577\u671F\u6709\u6548",
          "subscription": "\u8A02\u95B1\u8A08\u5283",
          "plans": "\u5957\u9910\u5217\u8868",
          "add_plan": "\u65B0\u589E\u5957\u9910",
          "subscription_none": "\u7121",
          "generate_count": "\u751F\u6210\u6578\u91CF",
          "generate_count_placeholder": "\u5982\u679C\u70BA\u6279\u91CF\u751F\u7522\u8ACB\u8F38\u5165\u751F\u6210\u6578\u91CF",
          "cancel": "\u53D6\u6D88",
          "submit": "\u751F\u6210",
          "success": "\u751F\u6210\u6210\u529F",
          "download_csv": "\u5C0E\u51FA\u70BA CSV \u6587\u4EF6",
          "generated_count": "\u5DF2\u751F\u6210 {{count}} \u500B\u8CEC\u865F",
          "single_success_hint": "\u8CEC\u865F\u5DF2\u5275\u5EFA\uFF0C\u53EF\u5728\u7528\u6236\u5217\u8868\u4E2D\u67E5\u770B\u8207\u7DE8\u8F2F\u3002",
          "copy_all": "\u8907\u88FD\u5168\u90E8",
          "copy_email": "\u8907\u88FD\u90F5\u7BB1",
          "copy_subscribe": "\u8907\u88FD\u8A02\u95B1",
          "result_password": "\u5BC6\u78BC",
          "result_expire": "\u5230\u671F"
        },
        "copy_line": {
          "email": "\u90F5\u7BB1: {{value}}",
          "password": "\u5BC6\u78BC: {{value}}",
          "expire": "\u5230\u671F: {{value}}",
          "subscribe": "\u8A02\u95B1: {{value}}"
        },
        "csv": {
          "email": "\u90F5\u7BB1",
          "password": "\u5BC6\u78BC",
          "expire_time": "\u5230\u671F\u6642\u9593",
          "uuid": "UUID",
          "created_at": "\u5275\u5EFA\u6642\u9593",
          "subscribe_url": "\u8A02\u95B1\u5730\u5740"
        }
      },
      "edit": {
        "button": "\u7DE8\u8F2F\u7528\u6236\u4FE1\u606F",
        "title": "\u7528\u6236\u7BA1\u7406",
        "form": {
          "email": "\u90F5\u7BB1",
          "phone_country": "\u570B\u5BB6/\u5730\u5340",
          "phone": "\u624B\u6A5F\u865F",
          "phone_placeholder": "\u8ACB\u8F38\u5165\u624B\u6A5F\u865F",
          "email_placeholder": "\u8ACB\u8F38\u5165\u90F5\u7BB1",
          "inviter_email": "\u9080\u8ACB\u4EBA\u90F5\u7BB1",
          "inviter_email_placeholder": "\u8ACB\u8F38\u5165\u90F5\u7BB1",
          "invite_user_id": "\u9080\u8ACB\u4EBAID",
          "invite_user_id_placeholder": "\u8ACB\u8F38\u5165\u9080\u8ACB\u4EBA\u7528\u6236ID\uFF0C\u7559\u7A7A\u5247\u6E05\u7A7A",
          "password": "\u5BC6\u78BC",
          "password_placeholder": "\u5982\u9700\u4FEE\u6539\u5BC6\u78BC\u8ACB\u8F38\u5165",
          "balance": "\u9918\u984D",
          "balance_placeholder": "\u8ACB\u8F38\u5165\u9918\u984D",
          "commission_balance": "\u4F63\u91D1\u9918\u984D",
          "commission_balance_placeholder": "\u8ACB\u8F38\u5165\u4F63\u91D1\u9918\u984D",
          "upload": "\u5DF2\u7528\u4E0A\u884C",
          "upload_placeholder": "\u5DF2\u7528\u4E0A\u884C",
          "download": "\u5DF2\u7528\u4E0B\u884C",
          "download_placeholder": "\u5DF2\u7528\u4E0B\u884C",
          "total_traffic": "\u6D41\u91CF",
          "total_traffic_placeholder": "\u8ACB\u8F38\u5165\u6D41\u91CF",
          "expire_time": "\u5230\u671F\u6642\u9593",
          "expire_time_placeholder": "\u8ACB\u9078\u64C7\u7528\u6236\u5230\u671F\u65E5\u671F\uFF0C\u7559\u7A7A\u70BA\u9577\u671F\u6709\u6548",
          "expire_time_specific": "\u5177\u9AD4\u6642\u9593",
          "expire_time_today": "\u8A2D\u70BA\u7576\u5929\u7D50\u675F",
          "expire_time_permanent": "\u9577\u671F\u6709\u6548",
          "expire_time_1month": "\u4E00\u500B\u6708",
          "expire_time_3months": "\u4E09\u500B\u6708",
          "expire_time_confirm": "\u78BA\u5B9A",
          "subscription": "\u8A02\u95B1\u8A08\u5283",
          "subscription_none": "\u7121",
          "account_status": "\u8CEC\u6236\u72C0\u614B",
          "banned_hint": "\u7528\u6236\u7121\u6CD5\u767B\u9304\u3001\u7121\u6CD5\u4F7F\u7528\u8A02\u95B1\u670D\u52D9",
          "normal_hint": "\u7528\u6236\u53EF\u4EE5\u6B63\u5E38\u4F7F\u7528\u6240\u6709\u670D\u52D9",
          "commission_type": "\u4F63\u91D1\u985E\u578B",
          "commission_type_system": "\u8DDF\u96A8\u7CFB\u7D71\u8A2D\u7F6E",
          "commission_type_cycle": "\u5FAA\u74B0\u8FD4\u5229",
          "commission_type_onetime": "\u9996\u6B21\u8FD4\u5229",
          "commission_rate": "\u63A8\u85A6\u8FD4\u5229\u6BD4\u4F8B",
          "commission_rate_placeholder": "\u70BA\u7A7A\u5247\u8DDF\u96A8\u7AD9\u9EDE\u8A2D\u7F6E\u8FD4\u5229\u6BD4\u4F8B",
          "discount": "\u5C08\u4EAB\u6298\u6263\u6BD4\u4F8B",
          "discount_placeholder": "\u70BA\u7A7A\u5247\u4E0D\u4EAB\u53D7\u5C08\u4EAB\u6298\u6263",
          "speed_limit": "\u9650\u901F",
          "speed_limit_placeholder": "\u7559\u7A7A\u5247\u4E0D\u9650\u901F",
          "device_limit": "\u8A2D\u5099\u9650\u5236",
          "device_limit_placeholder": "\u7559\u7A7A\u5247\u4E0D\u9650\u5236",
          "is_admin": "\u662F\u5426\u7BA1\u7406\u54E1",
          "is_staff": "\u662F\u5426\u54E1\u5DE5",
          "remarks": "\u5099\u8A3B",
          "remarks_placeholder": "\u8ACB\u5728\u9019\u88E1\u8A18\u9304",
          "cancel": "\u53D6\u6D88",
          "submit": "\u63D0\u4EA4",
          "success": "\u4FEE\u6539\u6210\u529F"
        }
      },
      "actions": {
        "title": "\u64CD\u4F5C",
        "send_email": "\u767C\u9001\u90F5\u4EF6",
        "export_csv": "\u5C0E\u51FA CSV",
        "traffic_reset_stats": "\u6D41\u91CF\u91CD\u7F6E\u7D71\u8A08",
        "batch_ban": "\u6279\u91CF\u5C01\u7981",
        "confirm_ban": {
          "title": "\u78BA\u8A8D\u6279\u91CF\u5C01\u7981",
          "filtered_description": "\u6B64\u64CD\u4F5C\u5C07\u5C01\u7981\u6240\u6709\u7B26\u5408\u7576\u524D\u7BE9\u9078\u689D\u4EF6\u7684\u7528\u6236\u3002\u6B64\u64CD\u4F5C\u7121\u6CD5\u64A4\u92B7\u3002",
          "all_description": "\u6B64\u64CD\u4F5C\u5C07\u5C01\u7981\u7CFB\u7D71\u4E2D\u7684\u6240\u6709\u7528\u6236\u3002\u6B64\u64CD\u4F5C\u7121\u6CD5\u64A4\u92B7\u3002",
          "cancel": "\u53D6\u6D88",
          "confirm": "\u78BA\u8A8D\u5C01\u7981",
          "banning": "\u5C01\u7981\u4E2D..."
        }
      },
      "traffic_reset": {
        "title": "\u6D41\u91CF\u91CD\u7F6E",
        "description": "\u70BA\u7528\u6236 {{email}} \u91CD\u7F6E\u6D41\u91CF\u4F7F\u7528\u91CF",
        "tabs": {
          "reset": "\u91CD\u7F6E\u6D41\u91CF",
          "history": "\u91CD\u7F6E\u6B77\u53F2"
        },
        "user_info": "\u7528\u6236\u4FE1\u606F",
        "warning": {
          "title": "\u91CD\u8981\u63D0\u9192",
          "irreversible": "\u6D41\u91CF\u91CD\u7F6E\u64CD\u4F5C\u4E0D\u53EF\u9006\uFF0C\u8ACB\u8B39\u614E\u64CD\u4F5C",
          "reset_to_zero": "\u91CD\u7F6E\u5F8C\u7528\u6236\u7684\u4E0A\u50B3\u548C\u4E0B\u8F09\u6D41\u91CF\u5C07\u6E05\u96F6",
          "logged": "\u6240\u6709\u91CD\u7F6E\u64CD\u4F5C\u90FD\u6703\u88AB\u8A18\u9304\u5728\u7CFB\u7D71\u65E5\u8A8C\u4E2D"
        },
        "reason": {
          "label": "\u91CD\u7F6E\u539F\u56E0",
          "placeholder": "\u8ACB\u8F38\u5165\u91CD\u7F6E\u6D41\u91CF\u7684\u539F\u56E0\uFF08\u53EF\u9078\uFF09",
          "optional": "\u6B64\u5B57\u6BB5\u70BA\u53EF\u9078\u9805\uFF0C\u7528\u65BC\u8A18\u9304\u91CD\u7F6E\u539F\u56E0"
        },
        "confirm_reset": "\u78BA\u8A8D\u91CD\u7F6E",
        "resetting": "\u91CD\u7F6E\u4E2D...",
        "reset_success": "\u6D41\u91CF\u91CD\u7F6E\u6210\u529F",
        "reset_failed": "\u6D41\u91CF\u91CD\u7F6E\u5931\u6557",
        "history": {
          "summary": "\u91CD\u7F6E\u6982\u89BD",
          "reset_count": "\u91CD\u7F6E\u6B21\u6578",
          "last_reset": "\u6700\u5F8C\u91CD\u7F6E",
          "next_reset": "\u4E0B\u6B21\u91CD\u7F6E",
          "never": "\u5F9E\u672A\u91CD\u7F6E",
          "no_schedule": "\u7121\u5B9A\u6642\u91CD\u7F6E",
          "records": "\u91CD\u7F6E\u8A18\u9304",
          "recent_records": "\u6700\u8FD110\u6B21\u91CD\u7F6E\u8A18\u9304",
          "no_records": "\u66AB\u7121\u91CD\u7F6E\u8A18\u9304",
          "reset_time": "\u91CD\u7F6E\u6642\u9593",
          "traffic_cleared": "\u6E05\u9664\u6D41\u91CF"
        },
        "stats": {
          "title": "\u6D41\u91CF\u91CD\u7F6E\u7D71\u8A08",
          "description": "\u67E5\u770B\u7CFB\u7D71\u6D41\u91CF\u91CD\u7F6E\u7684\u7D71\u8A08\u4FE1\u606F",
          "time_range": "\u7D71\u8A08\u6642\u9593\u7BC4\u570D",
          "total_resets": "\u7E3D\u91CD\u7F6E\u6B21\u6578",
          "auto_resets": "\u81EA\u52D5\u91CD\u7F6E",
          "manual_resets": "\u624B\u52D5\u91CD\u7F6E",
          "cron_resets": "\u5B9A\u6642\u91CD\u7F6E",
          "in_period": "\u6700\u8FD1 {{days}} \u5929",
          "breakdown": "\u91CD\u7F6E\u985E\u578B\u5206\u4F48",
          "breakdown_description": "\u5404\u985E\u578B\u91CD\u7F6E\u64CD\u4F5C\u7684\u767E\u5206\u6BD4\u5206\u4F48",
          "auto_percentage": "\u81EA\u52D5\u91CD\u7F6E\u4F54\u6BD4",
          "manual_percentage": "\u624B\u52D5\u91CD\u7F6E\u4F54\u6BD4",
          "cron_percentage": "\u5B9A\u6642\u91CD\u7F6E\u4F54\u6BD4",
          "days_options": {
            "week": "\u6700\u8FD1\u4E00\u9031",
            "month": "\u6700\u8FD1\u4E00\u6708",
            "quarter": "\u6700\u8FD1\u4E09\u6708",
            "year": "\u6700\u8FD1\u4E00\u5E74"
          }
        }
      },
      "traffic_reset_logs": {
        "title": "\u6D41\u91CF\u91CD\u7F6E\u65E5\u8A8C",
        "description": "\u67E5\u770B\u7CFB\u7D71\u4E2D\u6240\u6709\u6D41\u91CF\u91CD\u7F6E\u64CD\u4F5C\u7684\u8A73\u7D30\u8A18\u9304",
        "columns": {
          "id": "\u65E5\u8A8CID",
          "user": "\u7528\u6236",
          "reset_type": "\u91CD\u7F6E\u985E\u578B",
          "trigger_source": "\u89F8\u767C\u6E90",
          "cleared_traffic": "\u6E05\u9664\u6D41\u91CF",
          "cleared": "\u5DF2\u6E05\u9664",
          "upload": "\u4E0A\u50B3",
          "download": "\u4E0B\u8F09",
          "reset_time": "\u91CD\u7F6E\u6642\u9593",
          "log_time": "\u8A18\u9304\u6642\u9593"
        },
        "filters": {
          "search_user": "\u641C\u7D22\u7528\u6236\u90F5\u7BB1...",
          "reset_type": "\u91CD\u7F6E\u985E\u578B",
          "trigger_source": "\u89F8\u767C\u6E90",
          "all_types": "\u5168\u90E8\u985E\u578B",
          "all_sources": "\u5168\u90E8\u4F86\u6E90",
          "start_date": "\u958B\u59CB\u65E5\u671F",
          "end_date": "\u7D50\u675F\u65E5\u671F",
          "apply_date": "\u61C9\u7528\u7BE9\u9078",
          "reset": "\u91CD\u7F6E\u7BE9\u9078",
          "filter_title": "\u7BE9\u9078\u689D\u4EF6",
          "filter_description": "\u8A2D\u7F6E\u7BE9\u9078\u689D\u4EF6\u4F86\u67E5\u627E\u7279\u5B9A\u7684\u6D41\u91CF\u91CD\u7F6E\u8A18\u9304",
          "reset_types": {
            "monthly": "\u6309\u6708\u91CD\u7F6E",
            "first_day_month": "\u6BCF\u67081\u865F\u91CD\u7F6E",
            "yearly": "\u6309\u5E74\u91CD\u7F6E",
            "first_day_year": "\u6BCF\u5E741\u67081\u65E5\u91CD\u7F6E",
            "manual": "\u624B\u52D5\u91CD\u7F6E"
          },
          "trigger_sources": {
            "auto": "\u81EA\u52D5\u89F8\u767C",
            "manual": "\u624B\u52D5\u89F8\u767C",
            "cron": "\u5B9A\u6642\u4EFB\u52D9"
          }
        },
        "actions": {
          "export": "\u5C0E\u51FA\u65E5\u8A8C",
          "exporting": "\u5C0E\u51FA\u4E2D...",
          "export_success": "\u5C0E\u51FA\u6210\u529F",
          "export_failed": "\u5C0E\u51FA\u5931\u6557"
        },
        "trigger_descriptions": {
          "manual": "\u7BA1\u7406\u54E1\u624B\u52D5\u57F7\u884C\u7684\u6D41\u91CF\u91CD\u7F6E",
          "cron": "\u7CFB\u7D71\u5B9A\u6642\u4EFB\u52D9\u81EA\u52D5\u57F7\u884C",
          "auto": "\u7CFB\u7D71\u6839\u64DA\u689D\u4EF6\u81EA\u52D5\u89F8\u767C",
          "other": "\u5176\u4ED6\u65B9\u5F0F\u89F8\u767C"
        }
      },
      "login_history": {
        "title": "\u767B\u9304\u6B77\u53F2",
        "no_records": "\u66AB\u7121\u767B\u9304\u8A18\u9304",
        "columns": {
          "time": "\u6642\u9593",
          "ip": "IP",
          "method": "\u65B9\u5F0F",
          "user_agent": "User-Agent"
        },
        "methods": {
          "password": "\u5BC6\u78BC\u767B\u9304",
          "register": "\u8A3B\u518A",
          "mail_link": "\u90F5\u4EF6\u93C8\u63A5"
        }
      },
      "messages": {
        "success": "\u6210\u529F",
        "error": "\u932F\u8AA4",
        "export": {
          "success": "\u5C0E\u51FA\u6210\u529F",
          "failed": "\u5C0E\u51FA\u5931\u6557"
        },
        "batch_ban": {
          "success": "\u6279\u91CF\u5C01\u7981\u6210\u529F",
          "failed": "\u6279\u91CF\u5C01\u7981\u5931\u6557"
        },
        "send_mail": {
          "success": "\u90F5\u4EF6\u767C\u9001\u6210\u529F",
          "failed": "\u90F5\u4EF6\u767C\u9001\u5931\u6557",
          "required_fields": "\u8ACB\u586B\u5BEB\u6240\u6709\u5FC5\u586B\u5B57\u6BB5"
        },
        "reset_secret": {
          "success": "UUID & Token \u5DF2\u91CD\u7F6E"
        }
      },
      "send_mail": {
        "title": "\u767C\u9001\u90F5\u4EF6",
        "description": "\u5411\u6240\u9078\u6216\u5DF2\u7BE9\u9078\u7684\u7528\u6236\u767C\u9001\u90F5\u4EF6",
        "subject": "\u4E3B\u984C",
        "content": "\u5167\u5BB9",
        "sending": "\u767C\u9001\u4E2D...",
        "send": "\u767C\u9001"
      },
      "dialog": {
        "title": "\u7528\u6236\u8A73\u60C5",
        "basicInfo": "\u57FA\u672C\u4FE1\u606F",
        "subscriptionInfo": "\u8A02\u95B1\u4FE1\u606F",
        "trafficInfo": "\u6D41\u91CF\u4FE1\u606F",
        "financialInfo": "\u8CA1\u52D9\u4FE1\u606F",
        "activityInfo": "\u6D3B\u52D5\u4FE1\u606F",
        "inviteInfo": "\u9080\u8ACB\u4FE1\u606F",
        "timeInfo": "\u6642\u9593\u4FE1\u606F",
        "subscriptionUrl": "\u8A02\u95B1\u93C8\u63A5",
        "fields": {
          "userId": "\u7528\u6236ID",
          "email": "\u90F5\u7BB1",
          "phone": "\u624B\u6A5F\u865F",
          "uuid": "UUID",
          "token": "Token",
          "remarks": "\u5099\u8A3B",
          "subscriptionPlan": "\u8A02\u95B1\u5957\u9910",
          "permissionGroup": "\u6B0A\u9650\u7D44",
          "expiredAt": "\u5230\u671F\u6642\u9593",
          "deviceLimit": "\u8A2D\u5099\u9650\u5236",
          "speedLimit": "\u901F\u5EA6\u9650\u5236",
          "transferEnable": "\u7E3D\u6D41\u91CF",
          "uploadUsed": "\u4E0A\u50B3\u5DF2\u7528",
          "downloadUsed": "\u4E0B\u8F09\u5DF2\u7528",
          "totalUsed": "\u7E3D\u5DF2\u7528",
          "lastResetAt": "\u4E0A\u6B21\u91CD\u7F6E",
          "nextResetAt": "\u4E0B\u6B21\u91CD\u7F6E",
          "resetCount": "\u91CD\u7F6E\u6B21\u6578",
          "balance": "\u9918\u984D",
          "commissionBalance": "\u4F63\u91D1\u9918\u984D",
          "commissionType": "\u4F63\u91D1\u985E\u578B",
          "commissionRate": "\u4F63\u91D1\u6BD4\u4F8B",
          "lastLoginAt": "\u6700\u5F8C\u767B\u9304",
          "lastLoginIp": "\u6700\u5F8C\u767B\u9304IP",
          "registerIp": "\u8A3B\u518AIP",
          "lastOnlineAt": "\u6700\u5F8C\u5728\u7DDA",
          "onlineCount": "\u5728\u7DDA\u8A2D\u5099",
          "inviteUser": "\u9080\u8ACB\u4EBA",
          "inviteUserId": "\u9080\u8ACB\u4EBAID",
          "createdAt": "\u5275\u5EFA\u6642\u9593",
          "updatedAt": "\u66F4\u65B0\u6642\u9593",
          "subscribeUrl": "\u8A02\u95B1\u93C8\u63A5",
          "telegramId": "Telegram ID"
        }
      },
      "status": {
        "normal": "\u6B63\u5E38",
        "banned": "\u5DF2\u5C01\u7981",
        "admin": "\u7BA1\u7406\u54E1",
        "staff": "\u54E1\u5DE5"
      }
    },
    "nav": {
      "dashboard": "\u5100\u8868\u76E4",
      "systemManagement": "\u7CFB\u7D71\u7BA1\u7406",
      "systemConfig": "\u7CFB\u7D71\u914D\u7F6E",
      "pluginManagement": "\u63D2\u4EF6\u7BA1\u7406",
      "pluginMenuDemo": "\u63D2\u4EF6\u83DC\u55AE\uFF08\u6F14\u793A\uFF09",
      "noticeManagement": "\u516C\u544A\u7BA1\u7406",
      "paymentConfig": "\u652F\u4ED8\u914D\u7F6E",
      "knowledgeManagement": "\u77E5\u8B58\u5EAB\u7BA1\u7406",
      "nodeManagement": "\u7BC0\u9EDE\u7BA1\u7406",
      "machineManagement": "\u670D\u52D9\u5668\u7BA1\u7406",
      "permissionGroupManagement": "\u6B0A\u9650\u7D44\u7BA1\u7406",
      "routeManagement": "\u8DEF\u7531\u7BA1\u7406",
      "subscriptionManagement": "\u8A02\u95B1\u7BA1\u7406",
      "planManagement": "\u5957\u9910\u7BA1\u7406",
      "orderManagement": "\u8A02\u55AE\u7BA1\u7406",
      "couponManagement": "\u512A\u60E0\u5238\u7BA1\u7406",
      "giftCardManagement": "\u79AE\u54C1\u5361\u7BA1\u7406",
      "userManagement": "\u7528\u6236\u7BA1\u7406",
      "ticketManagement": "\u5DE5\u55AE\u7BA1\u7406",
      "withdrawalManagement": "\u63D0\u73B0\u7BA1\u7406",
      "trafficResetLogs": "\u6D41\u91CF\u91CD\u7F6E\u65E5\u8A8C",
      "pluginApps": "\u63D2\u4EF6\u61C9\u7528"
    },
    "subscribe": {
      "plan": {
        "title": "\u8A02\u95B1\u5957\u9910",
        "add": "\u6DFB\u52A0\u5957\u9910",
        "search": "\u641C\u7D22\u5957\u9910...",
        "sort": {
          "edit": "\u7DE8\u8F2F\u6392\u5E8F",
          "save": "\u4FDD\u5B58\u6392\u5E8F"
        },
        "columns": {
          "id": "ID",
          "show": "\u986F\u793A",
          "sell": "\u65B0\u8CFC",
          "renew": "\u7E8C\u8CBB",
          "renew_tooltip": "\u5728\u8A02\u95B1\u505C\u6B62\u92B7\u552E\u6642\uFF0C\u5DF2\u8CFC\u7528\u6236\u662F\u5426\u53EF\u4EE5\u7E8C\u8CBB",
          "name": "\u540D\u7A31",
          "users": "\u7E3D\u7528\u6236",
          "active_users": "\u6D3B\u8E8D\u7528\u6236",
          "stats": "\u7D71\u8A08",
          "group": "\u6B0A\u9650\u7D44",
          "price": "\u50F9\u683C",
          "actions": "\u64CD\u4F5C",
          "edit": "\u7DE8\u8F2F",
          "delete": "\u522A\u9664",
          "delete_confirm": {
            "title": "\u78BA\u8A8D\u522A\u9664",
            "description": "\u6B64\u64CD\u4F5C\u5C07\u6C38\u4E45\u522A\u9664\u8A72\u8A02\u95B1\uFF0C\u522A\u9664\u5F8C\u7121\u6CD5\u6062\u5FA9\u3002\u78BA\u5B9A\u8981\u7E7C\u7E8C\u55CE\uFF1F",
            "success": "\u522A\u9664\u6210\u529F"
          },
          "price_period": {
            "hourly": "\u6642\u4ED8",
            "daily": "\u65E5\u4ED8",
            "monthly": "\u6708\u4ED8",
            "quarterly": "\u5B63\u4ED8",
            "half_yearly": "\u534A\u5E74\u4ED8",
            "yearly": "\u5E74\u4ED8",
            "two_yearly": "\u5169\u5E74\u4ED8",
            "three_yearly": "\u4E09\u5E74\u4ED8",
            "onetime": "\u6D41\u91CF\u5305",
            "reset_traffic": "\u91CD\u7F6E\u5305",
            "no_price": "\u7121\u50F9\u683C",
            "unit": {
              "hour": "\u5143/\u6642",
              "day": "\u5143/\u5929",
              "month": "\u5143/\u6708",
              "quarter": "\u5143/\u5B63",
              "half_year": "\u5143/\u534A\u5E74",
              "year": "\u5143/\u5E74",
              "two_year": "\u5143/\u5169\u5E74",
              "three_year": "\u5143/\u4E09\u5E74",
              "times": "\u5143/\u6B21"
            }
          }
        },
        "form": {
          "add_title": "\u6DFB\u52A0\u5957\u9910",
          "edit_title": "\u7DE8\u8F2F\u5957\u9910",
          "name": {
            "label": "\u5957\u9910\u540D\u7A31",
            "placeholder": "\u8ACB\u8F38\u5165\u5957\u9910\u540D\u7A31",
            "required": "\u8ACB\u8F38\u5165\u5957\u9910\u540D\u7A31"
          },
          "group": {
            "label": "\u6B0A\u9650\u7D44",
            "add": "\u6DFB\u52A0\u6B0A\u9650\u7D44",
            "placeholder": "\u8ACB\u9078\u64C7\u6B0A\u9650\u7D44",
            "none": "\u4E0D\u7D81\u5B9A\u6B0A\u9650\u7D44"
          },
          "transfer": {
            "label": "\u6D41\u91CF",
            "placeholder": "\u8ACB\u8F38\u5165\u6D41\u91CF\u9650\u5236",
            "unit": "GB",
            "hint": "\u5957\u9910\u5305\u542B\u7684\u6D41\u91CF\u914D\u984D\uFF0C\u55AE\u4F4D\u70BA GB"
          },
          "speed": {
            "label": "\u901F\u5EA6\u9650\u5236",
            "placeholder": "0 \u8868\u793A\u4E0D\u9650\u5236",
            "unit": "Mbps",
            "hint": "\u55AE\u7528\u6236\u9650\u901F\uFF0C0 \u6216\u7559\u7A7A\u8868\u793A\u4E0D\u9650\u5236"
          },
          "price": {
            "title": "\u50F9\u683C\u8A2D\u7F6E",
            "base_price": "\u57FA\u790E\u50F9\u683C",
            "clear": {
              "button": "\u6E05\u7A7A",
              "tooltip": "\u6E05\u7A7A\u6240\u6709\u50F9\u683C"
            },
            "period": {
              "monthly": "\u6BCF\u6708",
              "months": "{{count}}\u500B\u6708"
            },
            "onetime_desc": "\u4E00\u6B21\u6027\u6D41\u91CF\u5305\uFF0C\u7121\u6642\u9593\u9650\u5236",
            "reset_desc": "\u91CD\u7F6E\u6D41\u91CF\u5305\uFF0C\u53EF\u591A\u6B21\u4F7F\u7528"
          },
          "device": {
            "label": "\u8A2D\u5099\u9650\u5236",
            "placeholder": "0 \u8868\u793A\u4E0D\u9650\u5236",
            "unit": "\u81FA",
            "hint": "\u540C\u6642\u5728\u7DDA\u8A2D\u5099\u6578\uFF0C0 \u6216\u7559\u7A7A\u8868\u793A\u4E0D\u9650\u5236"
          },
          "capacity": {
            "label": "\u5BB9\u91CF\u9650\u5236",
            "placeholder": "0 \u8868\u793A\u4E0D\u9650\u5236",
            "unit": "\u4EBA",
            "hint": "\u53EF\u552E\u8CE3\u540D\u984D\u4E0A\u9650\uFF0C0 \u6216\u7559\u7A7A\u8868\u793A\u4E0D\u9650\u5236"
          },
          "tags": {
            "label": "\u6A19\u7C64",
            "placeholder": "\u8F38\u5165\u6A19\u7C64\u5F8C\u6309\u56DE\u8ECA\u78BA\u8A8D"
          },
          "reset_method": {
            "label": "\u6D41\u91CF\u91CD\u7F6E\u65B9\u5F0F",
            "placeholder": "\u8ACB\u9078\u64C7\u91CD\u7F6E\u65B9\u5F0F",
            "description": "\u6D41\u91CF\u91CD\u7F6E\u65B9\u5F0F\u5C07\u6C7A\u5B9A\u5982\u4F55\u91CD\u7F6E\u6D41\u91CF",
            "options": {
              "follow_system": "\u8DDF\u96A8\u7CFB\u7D71\u8A2D\u7F6E",
              "monthly_first": "\u6BCF\u6708\u9996\u65E5",
              "monthly_reset": "\u6BCF\u6708\u8CFC\u8CB7\u65E5",
              "no_reset": "\u4E0D\u91CD\u7F6E",
              "yearly_first": "\u6BCF\u5E74\u9996\u65E5",
              "yearly_reset": "\u6BCF\u5E74\u8CFC\u8CB7\u65E5"
            }
          },
          "content": {
            "label": "\u5957\u9910\u8AAA\u660E",
            "placeholder": "\u8ACB\u8F38\u5165\u5957\u9910\u8AAA\u660E",
            "description": "\u652F\u6301 Markdown \u683C\u5F0F",
            "preview": "\u9810\u89BD",
            "preview_button": {
              "show": "\u986F\u793A\u9810\u89BD",
              "hide": "\u96B1\u85CF\u9810\u89BD"
            },
            "template": {
              "button": "\u4F7F\u7528\u6A21\u677F",
              "tooltip": "\u4F7F\u7528\u9ED8\u8A8D\u6A21\u677F",
              "content": "## \u5957\u9910\u8A73\u60C5\n\n- \u6D41\u91CF\uFF1A{{transfer}} GB\n- \u901F\u5EA6\u9650\u5236\uFF1A{{speed}} Mbps\n- \u540C\u6642\u5728\u7DDA\u8A2D\u5099\uFF1A{{devices}} \u81FA\n\n## \u670D\u52D9\u8AAA\u660E\n\n1. \u6D41\u91CF{{reset_method}}\u91CD\u7F6E\n2. \u652F\u6301\u591A\u5E73\u81FA\u4F7F\u7528\n3. 7\xD724\u5C0F\u6642\u6280\u8853\u652F\u6301"
            }
          },
          "section": {
            "basic": "\u57FA\u672C\u4FE1\u606F",
            "limits": "\u914D\u984D\u8207\u9650\u5236",
            "content": "\u5957\u9910\u8AAA\u660E",
            "status": "\u4E0A\u67B6\u72C0\u614B"
          },
          "status": {
            "show_desc": "\u5728\u524D\u81FA\u5957\u9910\u5217\u8868\u4E2D\u5C55\u793A",
            "sell_desc": "\u5141\u8A31\u65B0\u7528\u6236\u8CFC\u8CB7\u6B64\u5957\u9910",
            "renew_desc": "\u5141\u8A31\u5DF2\u6709\u7528\u6236\u7E8C\u8CBB\u6B64\u5957\u9910"
          },
          "force_update": {
            "label": "\u5F37\u5236\u66F4\u65B0\u7528\u6236\u5957\u9910",
            "description": "\u958B\u555F\u5F8C\u4FDD\u5B58\u6642\u6703\u5C07\u8A72\u5957\u9910\u4E0B\u7528\u6236\u7684\u6B0A\u9650\u7D44\u3001\u6D41\u91CF\u3001\u9650\u901F\u8207\u8A2D\u5099\u6578\u5F37\u5236\u540C\u6B65\u5230\u7576\u524D\u503C\uFF0C\u50C5\u5728\u7DE8\u8F2F\u5DF2\u6709\u5957\u9910\u6642\u751F\u6548\u3002"
          },
          "submit": {
            "cancel": "\u53D6\u6D88",
            "submit": "\u63D0\u4EA4",
            "submitting": "\u63D0\u4EA4\u4E2D...",
            "success": {
              "add": "\u5957\u9910\u6DFB\u52A0\u6210\u529F",
              "update": "\u5957\u9910\u66F4\u65B0\u6210\u529F"
            },
            "error": {
              "validation": "\u8868\u55AE\u6821\u9A57\u5931\u6557\uFF0C\u8ACB\u6AA2\u67E5\u4E26\u4FEE\u6B63\u932F\u8AA4\u5F8C\u91CD\u8A66\u3002"
            }
          }
        },
        "page": {
          "description": "\u5728\u9019\u88E1\u53EF\u4EE5\u914D\u7F6E\u8A02\u95B1\u8A08\u5283\uFF0C\u5305\u62EC\u6DFB\u52A0\u3001\u522A\u9664\u3001\u7DE8\u8F2F\u7B49\u64CD\u4F5C\u3002"
        }
      }
    },
    "settings": {
      "title": "\u7CFB\u7D71\u8A2D\u7F6E",
      "description": "\u7BA1\u7406\u7CFB\u7D71\u6838\u5FC3\u914D\u7F6E\uFF0C\u5305\u62EC\u7AD9\u9EDE\u3001\u5B89\u5168\u3001\u8A02\u95B1\u3001\u9080\u8ACB\u4F63\u91D1\u3001\u7BC0\u9EDE\u3001\u90F5\u4EF6\u548C\u901A\u77E5\u7B49\u8A2D\u7F6E",
      "site": {
        "title": "\u7AD9\u9EDE\u8A2D\u7F6E",
        "description": "\u914D\u7F6E\u7AD9\u9EDE\u57FA\u672C\u4FE1\u606F\uFF0C\u5305\u62EC\u7AD9\u9EDE\u540D\u7A31\u3001\u63CF\u8FF0\u3001\u8CA8\u5E63\u55AE\u4F4D\u7B49\u6838\u5FC3\u8A2D\u7F6E\u3002",
        "form": {
          "siteName": {
            "label": "\u7AD9\u9EDE\u540D\u7A31",
            "placeholder": "\u8ACB\u8F38\u5165\u7AD9\u9EDE\u540D\u7A31",
            "description": "\u7528\u65BC\u986F\u793A\u9700\u8981\u7AD9\u9EDE\u540D\u7A31\u7684\u5730\u65B9\u3002"
          },
          "logo": {
            "label": "\u7AD9\u9EDE LOGO",
            "placeholder": "\u8ACB\u8F38\u5165LOGO URL\uFF0C\u672B\u5C3E\u4E0D\u8981/",
            "description": "\u7528\u65BC\u986F\u793A\u9700\u8981LOGO\u7684\u5730\u65B9\u3002"
          },
          "siteDescription": {
            "label": "\u7AD9\u9EDE\u63CF\u8FF0",
            "placeholder": "\u8ACB\u8F38\u5165\u7AD9\u9EDE\u63CF\u8FF0",
            "description": "\u7528\u65BC\u986F\u793A\u9700\u8981\u7AD9\u9EDE\u63CF\u8FF0\u7684\u5730\u65B9\u3002"
          },
          "siteUrl": {
            "label": "\u7AD9\u9EDE\u7DB2\u5740",
            "placeholder": "\u8ACB\u8F38\u5165\u7AD9\u9EDEURL\uFF0C\u672B\u5C3E\u4E0D\u8981/",
            "description": "\u7576\u524D\u7DB2\u7AD9\u6700\u65B0\u7DB2\u5740\uFF0C\u5C07\u6703\u5728\u90F5\u4EF6\u7B49\u9700\u8981\u7528\u65BC\u7DB2\u5740\u8655\u9AD4\u73FE\u3002"
          },
          "forceHttps": {
            "label": "\u5F37\u5236HTTPS",
            "description": "\u7576\u7AD9\u9EDE\u6C92\u6709\u4F7F\u7528HTTPS\uFF0CCDN\u6216\u53CD\u4EE3\u958B\u555F\u5F37\u5236HTTPS\u6642\u9700\u8981\u958B\u555F\u3002"
          },
          "maintenanceMode": {
            "label": "\u7DAD\u8B77\u6A21\u5F0F",
            "description": "\u958B\u555F\u5F8C\u666E\u901A\u7528\u6236\u7684\u72C0\u614B\u8B8A\u66F4\u548C\u4EE3\u7406\u6D41\u91CF\u5C07\u66AB\u505C\uFF0C\u7BA1\u7406\u54E1\u4ECD\u53EF\u9032\u5165\u5F8C\u81FA\u95DC\u9589\u7DAD\u8B77\u6A21\u5F0F\u3002"
          },
          "subscribeUrl": {
            "label": "\u8A02\u95B1URL",
            "placeholder": "\u7528\u65BC\u8A02\u95B1\u6240\u4F7F\u7528\uFF0C\u591A\u500B\u8A02\u95B1\u5730\u5740\u7528','\u9694\u958B.\u7559\u7A7A\u5247\u70BA\u7AD9\u9EDEURL\u3002",
            "description": "\u7528\u65BC\u8A02\u95B1\u6240\u4F7F\u7528\uFF0C\u7559\u7A7A\u5247\u70BA\u7AD9\u9EDEURL\u3002"
          },
          "tosUrl": {
            "label": "\u7528\u6236\u689D\u6B3E(TOS)URL",
            "placeholder": "\u8ACB\u8F38\u5165\u7528\u6236\u689D\u6B3EURL\uFF0C\u672B\u5C3E\u4E0D\u8981/",
            "description": "\u7528\u65BC\u8DF3\u8F49\u5230\u7528\u6236\u689D\u6B3E(TOS)"
          },
          "stopRegister": {
            "label": "\u505C\u6B62\u65B0\u7528\u6236\u8A3B\u518A",
            "description": "\u958B\u555F\u5F8C\u4EFB\u4F55\u4EBA\u90FD\u5C07\u7121\u6CD5\u9032\u884C\u8A3B\u518A\u3002"
          },
          "ticketMustWaitReply": {
            "label": "\u5DE5\u55AE\u7B49\u5F85\u56DE\u8986\u9650\u5236",
            "description": "\u958B\u555F\u5F8C\uFF0C\u7528\u6236\u5728\u7BA1\u7406\u54E1\u56DE\u8986\u524D\u7121\u6CD5\u5728\u540C\u4E00\u5DE5\u55AE\u5167\u9023\u7E8C\u767C\u9001\u6D88\u606F\u3002"
          },
          "tryOut": {
            "label": "\u8A3B\u518A\u8A66\u7528\u5957\u9910",
            "placeholder": "\u8ACB\u9078\u64C7\u5957\u9910",
            "description": "\u9078\u64C7\u9700\u8981\u8A66\u7528\u7684\u8A02\u95B1\uFF0C\u5982\u679C\u6C92\u6709\u9078\u9805\u8ACB\u5148\u524D\u5F80\u8A02\u95B1\u7BA1\u7406\u6DFB\u52A0\u3002",
            "no_plan": "\u95DC\u9589",
            "duration": {
              "label": "\u8A3B\u518A\u8A66\u7528\u6642\u9577",
              "placeholder": "0",
              "description": "\u8A3B\u518A\u8A66\u7528\u6642\u9577\uFF0C\u55AE\u4F4D\u70BA\u5C0F\u6642\u3002"
            }
          },
          "currency": {
            "label": "\u8CA8\u5E63\u55AE\u4F4D",
            "placeholder": "CNY",
            "description": "\u50C5\u7528\u65BC\u5C55\u793A\u4F7F\u7528\uFF0C\u66F4\u6539\u5F8C\u7CFB\u7D71\u4E2D\u6240\u6709\u7684\u8CA8\u5E63\u55AE\u4F4D\u90FD\u5C07\u767C\u751F\u8B8A\u66F4\u3002"
          },
          "currencySymbol": {
            "label": "\u8CA8\u5E63\u7B26\u865F",
            "placeholder": "\xA5",
            "description": "\u50C5\u7528\u65BC\u5C55\u793A\u4F7F\u7528\uFF0C\u66F4\u6539\u5F8C\u7CFB\u7D71\u4E2D\u6240\u6709\u7684\u8CA8\u5E63\u55AE\u4F4D\u90FD\u5C07\u767C\u751F\u8B8A\u66F4\u3002"
          }
        }
      },
      "safe": {
        "title": "\u5B89\u5168\u8A2D\u7F6E",
        "description": "\u914D\u7F6E\u7CFB\u7D71\u5B89\u5168\u76F8\u95DC\u9078\u9805\uFF0C\u5305\u62EC\u767B\u9304\u9A57\u8B49\u3001\u5BC6\u78BC\u7B56\u7565\u3001API\u8A2A\u554F\u7B49\u5B89\u5168\u8A2D\u7F6E\u3002",
        "form": {
          "emailVerify": {
            "label": "\u90F5\u7BB1\u9A57\u8B49",
            "description": "\u958B\u555F\u5F8C\u5C07\u6703\u5F37\u5236\u8981\u6C42\u7528\u6236\u9032\u884C\u90F5\u7BB1\u9A57\u8B49\u3002"
          },
          "gmailLimit": {
            "label": "\u7981\u6B62\u4F7F\u7528Gmail\u591A\u5225\u540D",
            "description": "\u958B\u555F\u5F8CGmail\u591A\u5225\u540D\u5C07\u7121\u6CD5\u8A3B\u518A\u3002"
          },
          "safeMode": {
            "label": "\u5B89\u5168\u6A21\u5F0F",
            "description": "\u958B\u555F\u5F8C\u9664\u4E86\u7AD9\u9EDEURL\u4EE5\u5916\u7684\u7D81\u5B9A\u672C\u7AD9\u9EDE\u7684\u57DF\u540D\u8A2A\u554F\u90FD\u5C07\u6703\u88AB403\u3002"
          },
          "securePath": {
            "label": "\u5F8C\u81FA\u8DEF\u5F91",
            "placeholder": "admin",
            "description": "\u5F8C\u81FA\u7BA1\u7406\u8DEF\u5F91\uFF0C\u4FEE\u6539\u5F8C\u5C07\u6703\u6539\u8B8A\u539F\u6709\u7684admin\u8DEF\u5F91"
          },
          "emailWhitelist": {
            "label": "\u90F5\u7BB1\u5F8C\u7DB4\u767D\u540D\u55AE",
            "description": "\u958B\u555F\u5F8C\u5728\u540D\u55AE\u4E2D\u7684\u90F5\u7BB1\u5F8C\u7DB4\u624D\u5141\u8A31\u9032\u884C\u8A3B\u518A\u3002",
            "suffixes": {
              "label": "\u90F5\u7BB1\u5F8C\u7DB4",
              "placeholder": "\u8F38\u5165\u90F5\u7BB1\u5F8C\u7DB4\uFF0C\u6BCF\u884C\u4E00\u500B",
              "description": "\u8F38\u5165\u5141\u8A31\u7684\u90F5\u7BB1\u5F8C\u7DB4\uFF0C\u6BCF\u884C\u4E00\u500B"
            }
          },
          "captcha": {
            "enable": {
              "label": "\u555F\u7528\u9A57\u8B49\u78BC",
              "description": "\u958B\u555F\u5F8C\u7528\u6236\u8A3B\u518A\u6642\u9700\u8981\u901A\u904E\u9A57\u8B49\u78BC\u9A57\u8B49\u3002"
            },
            "type": {
              "label": "\u9A57\u8B49\u78BC\u985E\u578B",
              "description": "\u9078\u64C7\u8981\u4F7F\u7528\u7684\u9A57\u8B49\u78BC\u670D\u52D9\u985E\u578B",
              "options": {
                "recaptcha": "Google reCAPTCHA v2",
                "recaptcha-v3": "Google reCAPTCHA v3",
                "turnstile": "Cloudflare Turnstile"
              }
            },
            "recaptcha": {
              "key": {
                "label": "reCAPTCHA\u5BC6\u9470",
                "placeholder": "\u8F38\u5165reCAPTCHA\u5BC6\u9470",
                "description": "\u8F38\u5165\u60A8\u7684reCAPTCHA\u5BC6\u9470"
              },
              "siteKey": {
                "label": "reCAPTCHA\u7AD9\u9EDE\u5BC6\u9470",
                "placeholder": "\u8F38\u5165reCAPTCHA\u7AD9\u9EDE\u5BC6\u9470",
                "description": "\u8F38\u5165\u60A8\u7684reCAPTCHA\u7AD9\u9EDE\u5BC6\u9470"
              }
            },
            "recaptcha_v3": {
              "secretKey": {
                "label": "reCAPTCHA v3\u5BC6\u9470",
                "placeholder": "\u8F38\u5165reCAPTCHA v3\u5BC6\u9470",
                "description": "\u8F38\u5165\u60A8\u7684reCAPTCHA v3\u670D\u52D9\u5668\u5BC6\u9470"
              },
              "siteKey": {
                "label": "reCAPTCHA v3\u7AD9\u9EDE\u5BC6\u9470",
                "placeholder": "\u8F38\u5165reCAPTCHA v3\u7AD9\u9EDE\u5BC6\u9470",
                "description": "\u8F38\u5165\u60A8\u7684reCAPTCHA v3\u7AD9\u9EDE\u5BC6\u9470"
              },
              "scoreThreshold": {
                "label": "\u5206\u6578\u95BE\u503C",
                "placeholder": "0.5",
                "description": "\u8A2D\u7F6E\u9A57\u8B49\u5206\u6578\u95BE\u503C\uFF080-1\uFF09\uFF0C\u5206\u6578\u8D8A\u9AD8\u8868\u793A\u8D8A\u53EF\u80FD\u662F\u771F\u4EBA\u64CD\u4F5C"
              }
            },
            "turnstile": {
              "secretKey": {
                "label": "Turnstile\u5BC6\u9470",
                "placeholder": "\u8F38\u5165Turnstile\u5BC6\u9470",
                "description": "\u8F38\u5165\u60A8\u7684Cloudflare Turnstile\u5BC6\u9470"
              },
              "siteKey": {
                "label": "Turnstile\u7AD9\u9EDE\u5BC6\u9470",
                "placeholder": "\u8F38\u5165Turnstile\u7AD9\u9EDE\u5BC6\u9470",
                "description": "\u8F38\u5165\u60A8\u7684Cloudflare Turnstile\u7AD9\u9EDE\u5BC6\u9470"
              }
            }
          },
          "registerLimit": {
            "enable": {
              "label": "IP\u8A3B\u518A\u9650\u5236",
              "description": "\u958B\u555F\u5F8C\u5C07\u9650\u5236\u540C\u4E00IP\u7684\u8A3B\u518A\u6B21\u6578\u3002"
            },
            "count": {
              "label": "\u8A3B\u518A\u6B21\u6578",
              "placeholder": "\u8F38\u5165\u6700\u5927\u8A3B\u518A\u6B21\u6578",
              "description": "\u540C\u4E00IP\u5141\u8A31\u7684\u6700\u5927\u8A3B\u518A\u6B21\u6578"
            },
            "expire": {
              "label": "\u9650\u5236\u6642\u9577",
              "placeholder": "\u8F38\u5165\u9650\u5236\u6642\u9577\uFF08\u5206\u9418\uFF09",
              "description": "\u8A3B\u518A\u9650\u5236\u7684\u6301\u7E8C\u6642\u9593\uFF08\u5206\u9418\uFF09"
            }
          },
          "passwordLimit": {
            "enable": {
              "label": "\u5BC6\u78BC\u5617\u8A66\u9650\u5236",
              "description": "\u958B\u555F\u5F8C\u5C07\u9650\u5236\u5BC6\u78BC\u5617\u8A66\u6B21\u6578\u3002"
            },
            "count": {
              "label": "\u5617\u8A66\u6B21\u6578",
              "placeholder": "\u8F38\u5165\u6700\u5927\u5617\u8A66\u6B21\u6578",
              "description": "\u5141\u8A31\u7684\u6700\u5927\u5BC6\u78BC\u5617\u8A66\u6B21\u6578"
            },
            "expire": {
              "label": "\u9396\u5B9A\u6642\u9577",
              "placeholder": "\u8F38\u5165\u9396\u5B9A\u6642\u9577\uFF08\u5206\u9418\uFF09",
              "description": "\u8CEC\u6236\u9396\u5B9A\u7684\u6301\u7E8C\u6642\u9593\uFF08\u5206\u9418\uFF09"
            }
          }
        }
      },
      "subscribe": {
        "title": "\u8A02\u95B1\u8A2D\u7F6E",
        "description": "\u7BA1\u7406\u7528\u6236\u8A02\u95B1\u76F8\u95DC\u914D\u7F6E\uFF0C\u5305\u62EC\u8A02\u95B1\u93C8\u63A5\u683C\u5F0F\u3001\u66F4\u65B0\u983B\u7387\u3001\u6D41\u91CF\u7D71\u8A08\u7B49\u8A2D\u7F6E\u3002",
        "plan_change_enable": {
          "title": "\u5141\u8A31\u7528\u6236\u66F4\u6539\u8A02\u95B1",
          "description": "\u958B\u555F\u5F8C\u7528\u6236\u5C07\u6703\u53EF\u4EE5\u5C0D\u8A02\u95B1\u8A08\u5283\u9032\u884C\u8B8A\u66F4\u3002"
        },
        "reset_traffic_method": {
          "title": "\u6708\u6D41\u91CF\u91CD\u7F6E\u65B9\u5F0F",
          "description": "\u5168\u5C40\u6D41\u91CF\u91CD\u7F6E\u65B9\u5F0F\uFF0C\u9ED8\u8A8D\u6BCF\u67081\u865F\u3002\u53EF\u4EE5\u5728\u8A02\u95B1\u7BA1\u7406\u70BA\u8A02\u95B1\u55AE\u7368\u8A2D\u7F6E\u3002",
          "options": {
            "monthly_first": "\u6BCF\u67081\u865F",
            "monthly_reset": "\u6309\u6708\u91CD\u7F6E",
            "no_reset": "\u4E0D\u91CD\u7F6E",
            "yearly_first": "\u6BCF\u5E741\u67081\u865F",
            "yearly_reset": "\u6309\u5E74\u91CD\u7F6E"
          }
        },
        "surplus_enable": {
          "title": "\u958B\u555F\u6298\u62B5\u65B9\u6848",
          "description": "\u958B\u555F\u5F8C\u7528\u6236\u66F4\u63DB\u8A02\u95B1\u5C07\u6703\u7531\u7CFB\u7D71\u5C0D\u539F\u6709\u8A02\u95B1\u9032\u884C\u6298\u62B5\uFF0C\u65B9\u6848\u53C3\u8003\u6587\u6A94\u3002"
        },
        "new_order_event": {
          "title": "\u7576\u8A02\u95B1\u65B0\u8CFC\u6642\u89F8\u767C\u4E8B\u4EF6",
          "description": "\u65B0\u8CFC\u8A02\u95B1\u5B8C\u6210\u6642\u5C07\u89F8\u767C\u8A72\u4EFB\u52D9\u3002",
          "options": {
            "no_action": "\u4E0D\u57F7\u884C\u4EFB\u4F55\u52D5\u4F5C",
            "reset_traffic": "\u91CD\u7F6E\u7528\u6236\u6D41\u91CF"
          }
        },
        "renew_order_event": {
          "title": "\u7576\u8A02\u95B1\u7E8C\u8CBB\u6642\u89F8\u767C\u4E8B\u4EF6",
          "description": "\u7E8C\u8CBB\u8A02\u95B1\u5B8C\u6210\u6642\u5C07\u89F8\u767C\u8A72\u4EFB\u52D9\u3002",
          "options": {
            "no_action": "\u4E0D\u57F7\u884C\u4EFB\u4F55\u52D5\u4F5C",
            "reset_traffic": "\u91CD\u7F6E\u7528\u6236\u6D41\u91CF"
          }
        },
        "change_order_event": {
          "title": "\u7576\u8A02\u95B1\u8B8A\u66F4\u6642\u89F8\u767C\u4E8B\u4EF6",
          "description": "\u8B8A\u66F4\u8A02\u95B1\u5B8C\u6210\u6642\u5C07\u89F8\u767C\u8A72\u4EFB\u52D9\u3002",
          "options": {
            "no_action": "\u4E0D\u57F7\u884C\u4EFB\u4F55\u52D5\u4F5C",
            "reset_traffic": "\u91CD\u7F6E\u7528\u6236\u6D41\u91CF"
          }
        },
        "subscribe_path": {
          "title": "\u8A02\u95B1\u8DEF\u5F91",
          "description": "\u8A02\u95B1\u8DEF\u5F91\uFF0C\u4FEE\u6539\u5F8C\u5C07\u6703\u6539\u8B8A\u539F\u6709\u7684subscribe\u8DEF\u5F91",
          "current_format": "\u7576\u524D\u8A02\u95B1\u8DEF\u5F91\u683C\u5F0F\uFF1A{path}/xxxxxxxxxx",
          "restart_tip": "\u4FEE\u6539\u8A02\u95B1\u8DEF\u5F91\u5F8C\uFF0C\u53EF\u80FD\u9700\u8981\u91CD\u555F\u670D\u52D9\u624D\u80FD\u751F\u6548\u3002"
        },
        "show_info_to_server": {
          "title": "\u5728\u8A02\u95B1\u4E2D\u5C55\u793A\u8A02\u95B1\u4FE1\u606F",
          "description": "\u958B\u555F\u5F8C\u5C07\u6703\u5728\u7528\u6236\u8A02\u95B1\u7BC0\u9EDE\u6642\u8F38\u51FA\u8A02\u95B1\u4FE1\u606F\u3002"
        },
        "show_protocol_to_server": {
          "title": "\u5728\u8A02\u95B1\u4E2D\u7DDA\u8DEF\u540D\u7A31\u4E2D\u986F\u793A\u5354\u8B70\u540D\u7A31",
          "description": "\u958B\u555F\u5F8C\u8A02\u95B1\u7DDA\u8DEF\u6703\u9644\u5E36\u5354\u8B70\u540D\u7A31\uFF08\u4F8B\u5982: [Hy2]\u9999\u6E2F\uFF09"
        },
        "default_remind_expire": {
          "title": "\u65B0\u7528\u6236\u9ED8\u8A8D\u958B\u555F\u5230\u671F\u63D0\u9192",
          "description": "\u958B\u555F\u5F8C\u65B0\u8A3B\u518A\u7528\u6236\u9ED8\u8A8D\u555F\u7528\u8A02\u95B1\u5230\u671F\u63D0\u9192\uFF0C\u53EF\u5728\u7528\u6236\u7BA1\u7406\u55AE\u7368\u8ABF\u6574\u3002"
        },
        "default_remind_traffic": {
          "title": "\u65B0\u7528\u6236\u9ED8\u8A8D\u958B\u555F\u6D41\u91CF\u63D0\u9192",
          "description": "\u958B\u555F\u5F8C\u65B0\u8A3B\u518A\u7528\u6236\u9ED8\u8A8D\u555F\u7528\u6D41\u91CF\u4E0D\u8DB3\u63D0\u9192\uFF0C\u53EF\u5728\u7528\u6236\u7BA1\u7406\u55AE\u7368\u8ABF\u6574\u3002"
        },
        "deposit_enable": {
          "title": "\u958B\u555F\u9918\u984D\u5145\u503C",
          "description": "\u958B\u555F\u5F8C\u7528\u6236\u53EF\u5728\u524D\u53F0\u5EFA\u7ACB\u9918\u984D\u5145\u503C\u8A02\u55AE\u4E26\u5B8C\u6210\u652F\u4ED8\u5165\u5E33\u3002"
        },
        "deposit_commission_enable": {
          "title": "\u9918\u984D\u5145\u503C\u53C3\u8207\u9080\u8ACB\u8FD4\u4F63",
          "description": "\u958B\u555F\u5F8C\u9918\u984D\u5145\u503C\u8A02\u55AE\u5C07\u4F9D\u9080\u8ACB\u898F\u5247\u8A08\u7B97\u4F63\u91D1\u3002"
        },
        "deposit_min_amount": {
          "title": "\u6700\u4F4E\u5145\u503C\u91D1\u984D\uFF08\u5206\uFF09",
          "description": "\u7528\u6236\u55AE\u7B46\u5145\u503C\u7684\u6700\u4F4E\u91D1\u984D\uFF0C\u55AE\u4F4D\u70BA\u5206\u3002\u4F8B\u5982 100 \u8868\u793A 1 \u5143\u3002",
          "placeholder": "100"
        },
        "deposit_max_amount": {
          "title": "\u6700\u9AD8\u5145\u503C\u91D1\u984D\uFF08\u5206\uFF09",
          "description": "\u7528\u6236\u55AE\u7B46\u5145\u503C\u7684\u6700\u9AD8\u91D1\u984D\uFF0C\u55AE\u4F4D\u70BA\u5206\u3002",
          "placeholder": "999999900"
        },
        "deposit_bonus": {
          "title": "\u5145\u503C\u8D08\u9001\u968E\u68AF",
          "description": "\u683C\u5F0F\u70BA\u300C\u9580\u6ABB\u5143:\u8D08\u9001\u5143\u300D\uFF0C\u4F8B\u5982 100:10 \u8868\u793A\u5145\u503C\u6EFF 100 \u5143\u8D08\u9001 10 \u5143\u3002\u53D6\u6EFF\u8DB3\u9580\u6ABB\u7684\u6700\u5927\u8D08\u9001\u3002",
          "placeholder": "100:10"
        },
        "saving": "\u4FDD\u5B58\u4E2D...",
        "plan": {
          "title": "\u8A02\u95B1\u5957\u9910",
          "add": "\u6DFB\u52A0\u5957\u9910",
          "search": "\u641C\u7D22\u5957\u9910...",
          "sort": {
            "edit": "\u7DE8\u8F2F\u6392\u5E8F",
            "save": "\u4FDD\u5B58\u6392\u5E8F"
          },
          "columns": {
            "id": "\u7DE8\u865F",
            "show": "\u986F\u793A",
            "sell": "\u65B0\u8CFC",
            "renew": "\u7E8C\u8CBB",
            "renew_tooltip": "\u5728\u8A02\u95B1\u505C\u6B62\u92B7\u552E\u6642\uFF0C\u5DF2\u8CFC\u7528\u6236\u662F\u5426\u53EF\u4EE5\u7E8C\u8CBB",
            "name": "\u540D\u7A31",
            "stats": "\u7D71\u8A08",
            "group": "\u6B0A\u9650\u7D44",
            "price": "\u50F9\u683C",
            "actions": "\u64CD\u4F5C",
            "edit": "\u7DE8\u8F2F",
            "delete": "\u522A\u9664",
            "delete_confirm": {
              "title": "\u78BA\u8A8D\u522A\u9664",
              "description": "\u6B64\u64CD\u4F5C\u5C07\u6C38\u4E45\u522A\u9664\u8A72\u8A02\u95B1\uFF0C\u522A\u9664\u5F8C\u7121\u6CD5\u6062\u5FA9\u3002\u78BA\u5B9A\u8981\u7E7C\u7E8C\u55CE\uFF1F",
              "success": "\u522A\u9664\u6210\u529F"
            },
            "price_period": {
              "hourly": "\u6642\u4ED8",
              "daily": "\u65E5\u4ED8",
              "monthly": "\u6708\u4ED8",
              "quarterly": "\u5B63\u4ED8",
              "half_yearly": "\u534A\u5E74\u4ED8",
              "yearly": "\u5E74\u4ED8",
              "two_yearly": "\u5169\u5E74\u4ED8",
              "three_yearly": "\u4E09\u5E74\u4ED8",
              "onetime": "\u6D41\u91CF\u5305",
              "reset_traffic": "\u91CD\u7F6E\u5305",
              "unit": {
                "hour": "\u5143/\u6642",
                "day": "\u5143/\u5929",
                "month": "\u5143/\u6708",
                "quarter": "\u5143/\u5B63",
                "half_year": "\u5143/\u534A\u5E74",
                "year": "\u5143/\u5E74",
                "two_year": "\u5143/\u5169\u5E74",
                "three_year": "\u5143/\u4E09\u5E74",
                "times": "\u5143/\u6B21"
              }
            }
          },
          "form": {
            "add_title": "\u6DFB\u52A0\u5957\u9910",
            "edit_title": "\u7DE8\u8F2F\u5957\u9910",
            "name": {
              "label": "\u5957\u9910\u540D\u7A31",
              "placeholder": "\u8ACB\u8F38\u5165\u5957\u9910\u540D\u7A31"
            },
            "group": {
              "label": "\u6B0A\u9650\u7D44",
              "placeholder": "\u9078\u64C7\u6B0A\u9650\u7D44",
              "add": "\u6DFB\u52A0\u6B0A\u9650\u7D44",
              "none": "\u4E0D\u7D81\u5B9A\u6B0A\u9650\u7D44"
            },
            "transfer": {
              "label": "\u6D41\u91CF",
              "placeholder": "\u8ACB\u8F38\u5165\u6D41\u91CF\u5927\u5C0F",
              "unit": "GB"
            },
            "speed": {
              "label": "\u9650\u901F",
              "placeholder": "\u8ACB\u8F38\u5165\u9650\u901F",
              "unit": "Mbps"
            },
            "price": {
              "title": "\u552E\u50F9\u8A2D\u7F6E",
              "base_price": "\u57FA\u790E\u6708\u4ED8\u50F9\u683C",
              "clear": {
                "button": "\u6E05\u7A7A\u50F9\u683C",
                "tooltip": "\u6E05\u7A7A\u6240\u6709\u5468\u671F\u7684\u50F9\u683C\u8A2D\u7F6E"
              }
            },
            "device": {
              "label": "\u8A2D\u5099\u9650\u5236",
              "placeholder": "\u7559\u7A7A\u5247\u4E0D\u9650\u5236",
              "unit": "\u81FA"
            },
            "capacity": {
              "label": "\u5BB9\u91CF\u9650\u5236",
              "placeholder": "\u7559\u7A7A\u5247\u4E0D\u9650\u5236",
              "unit": "\u4EBA"
            },
            "reset_method": {
              "label": "\u6D41\u91CF\u91CD\u7F6E\u65B9\u5F0F",
              "placeholder": "\u9078\u64C7\u6D41\u91CF\u91CD\u7F6E\u65B9\u5F0F",
              "description": "\u8A2D\u7F6E\u8A02\u95B1\u6D41\u91CF\u7684\u91CD\u7F6E\u65B9\u5F0F\uFF0C\u4E0D\u540C\u7684\u91CD\u7F6E\u65B9\u5F0F\u6703\u5F71\u97FF\u7528\u6236\u7684\u6D41\u91CF\u8A08\u7B97\u65B9\u5F0F",
              "options": {
                "follow_system": "\u8DDF\u96A8\u7CFB\u7D71\u8A2D\u7F6E",
                "monthly_first": "\u6BCF\u67081\u865F",
                "monthly_reset": "\u6309\u6708\u91CD\u7F6E",
                "no_reset": "\u4E0D\u91CD\u7F6E",
                "yearly_first": "\u6BCF\u5E741\u67081\u65E5",
                "yearly_reset": "\u6309\u5E74\u91CD\u7F6E"
              }
            },
            "content": {
              "label": "\u5957\u9910\u63CF\u8FF0",
              "placeholder": "\u5728\u9019\u88E1\u7DE8\u5BEB\u5957\u9910\u63CF\u8FF0...",
              "description": "\u652F\u6301 Markdown \u683C\u5F0F\uFF0C\u53EF\u4EE5\u4F7F\u7528\u6A19\u984C\u3001\u5217\u8868\u3001\u7C97\u9AD4\u3001\u659C\u9AD4\u7B49\u6A23\u5F0F\u4F86\u7F8E\u5316\u63CF\u8FF0\u5167\u5BB9",
              "preview": "\u9810\u89BD",
              "preview_button": {
                "show": "\u986F\u793A\u9810\u89BD",
                "hide": "\u96B1\u85CF\u9810\u89BD"
              },
              "template": {
                "button": "\u4F7F\u7528\u6A21\u677F",
                "tooltip": "\u9EDE\u64CA\u4F7F\u7528\u9810\u8A2D\u7684\u5957\u9910\u63CF\u8FF0\u6A21\u677F",
                "content": "## \u5957\u9910\u7279\u9EDE\n\u2022 \u9AD8\u901F\u7A69\u5B9A\u7684\u5168\u7403\u7DB2\u7D61\u63A5\u5165\n\u2022 \u652F\u6301\u591A\u8A2D\u5099\u540C\u6642\u5728\u7DDA\n\u2022 \u7121\u9650\u5236\u7684\u6D41\u91CF\u91CD\u7F6E\n\n## \u4F7F\u7528\u8AAA\u660E\n1. \u652F\u6301\u8A2D\u5099\uFF1AiOS\u3001Android\u3001Windows\u3001macOS\n2. 24/7 \u6280\u8853\u652F\u6301\n3. \u81EA\u52D5\u5B9A\u671F\u6D41\u91CF\u91CD\u7F6E\n\n## \u6CE8\u610F\u4E8B\u9805\n- \u7981\u6B62\u6FEB\u7528\n- \u9075\u5B88\u7576\u5730\u6CD5\u5F8B\u6CD5\u898F\n- \u652F\u6301\u96A8\u6642\u66F4\u63DB\u5957\u9910"
              }
            },
            "force_update": {
              "label": "\u5F37\u5236\u66F4\u65B0\u5230\u7528\u6236",
              "description": "\u958B\u555F\u5F8C\u4FDD\u5B58\u6642\u5C07\u8A72\u5957\u9910\u4E0B\u7528\u6236\u7684\u6B0A\u9650\u7D44\u3001\u6D41\u91CF\u3001\u9650\u901F\u8207\u8A2D\u5099\u6578\u5F37\u5236\u540C\u6B65\u5230\u7576\u524D\u503C\u3002"
            },
            "submit": {
              "submitting": "\u63D0\u4EA4\u4E2D...",
              "submit": "\u63D0\u4EA4",
              "cancel": "\u53D6\u6D88",
              "success": {
                "add": "\u5957\u9910\u6DFB\u52A0\u6210\u529F",
                "update": "\u5957\u9910\u66F4\u65B0\u6210\u529F"
              }
            }
          },
          "page": {
            "description": "\u5728\u9019\u88E1\u53EF\u4EE5\u914D\u7F6E\u8A02\u95B1\u8A08\u5283\uFF0C\u5305\u62EC\u6DFB\u52A0\u3001\u522A\u9664\u3001\u7DE8\u8F2F\u7B49\u64CD\u4F5C\u3002"
          }
        }
      },
      "email": {
        "title": "\u90F5\u4EF6\u8A2D\u7F6E",
        "description": "\u914D\u7F6E\u7CFB\u7D71\u90F5\u4EF6\u670D\u52D9\uFF0C\u7528\u65BC\u767C\u9001\u9A57\u8B49\u78BC\u3001\u5BC6\u78BC\u91CD\u7F6E\u3001\u901A\u77E5\u7B49\u90F5\u4EF6\uFF0C\u652F\u6301\u591A\u7A2ESMTP\u670D\u52D9\u5546\u3002",
        "tab_settings": "\u57FA\u672C\u8A2D\u7F6E",
        "tab_templates": "\u6A21\u677F\u7BA1\u7406",
        "email_host": {
          "title": "SMTP\u4E3B\u6A5F",
          "description": "SMTP\u670D\u52D9\u5668\u5730\u5740\uFF0C\u4F8B\u5982\uFF1Asmtp.gmail.com",
          "placeholder": "smtp.example.com"
        },
        "email_port": {
          "title": "SMTP\u7AEF\u53E3",
          "description": "SMTP\u670D\u52D9\u5668\u7AEF\u53E3\uFF0C\u5E38\u7528\u7AEF\u53E3\uFF1A25, 465, 587",
          "placeholder": "465"
        },
        "email_username": {
          "title": "SMTP\u7528\u6236\u540D",
          "description": "SMTP\u8A8D\u8B49\u7528\u6236\u540D",
          "placeholder": "user@example.com"
        },
        "email_password": {
          "title": "SMTP\u5BC6\u78BC",
          "description": "SMTP\u8A8D\u8B49\u5BC6\u78BC\u6216\u61C9\u7528\u5C08\u7528\u5BC6\u78BC",
          "placeholder": "\u8ACB\u8F38\u5165\u5BC6\u78BC\u6216\u61C9\u7528\u5C08\u7528\u5BC6\u78BC"
        },
        "email_encryption": {
          "title": "\u52A0\u5BC6\u65B9\u5F0F",
          "description": "\u90F5\u4EF6\u52A0\u5BC6\u65B9\u5F0F",
          "placeholder": "\u9078\u64C7\u52A0\u5BC6\u65B9\u5F0F",
          "none": "\u7121",
          "ssl": "SSL/TLS",
          "tls": "STARTTLS"
        },
        "email_from_address": {
          "title": "\u767C\u4EF6\u4EBA\u5730\u5740",
          "description": "\u767C\u4EF6\u4EBA\u90F5\u7BB1\u5730\u5740",
          "placeholder": "noreply@example.com"
        },
        "email_from_name": {
          "title": "\u767C\u4EF6\u4EBA\u540D\u7A31",
          "description": "\u767C\u4EF6\u4EBA\u986F\u793A\u540D\u7A31"
        },
        "email_template": {
          "title": "\u90F5\u4EF6\u6A21\u677F",
          "description": "\u81EA\u5B9A\u7FA9\u90F5\u4EF6\u6A21\u677F\u65B9\u5F0F\u8ACB\u67E5\u770B\u6587\u6A94",
          "placeholder": "\u9078\u64C7\u90F5\u4EF6\u6A21\u677F"
        },
        "remind_mail_enable": {
          "title": "\u90F5\u4EF6\u63D0\u9192",
          "description": "\u958B\u555F\u5F8C\u7528\u6236\u8A02\u95B1\u5373\u5C07\u5230\u671F\u6216\u6D41\u91CF\u4E0D\u8DB3\u6642\u6703\u6536\u5230\u90F5\u4EF6\u901A\u77E5\u3002"
        },
        "test": {
          "title": "\u767C\u9001\u6E2C\u8A66\u90F5\u4EF6",
          "sending": "\u767C\u9001\u4E2D...",
          "description": "\u767C\u9001\u6E2C\u8A66\u90F5\u4EF6\u4EE5\u9A57\u8B49\u914D\u7F6E",
          "success": "\u6E2C\u8A66\u90F5\u4EF6\u767C\u9001\u6210\u529F",
          "error": "\u6E2C\u8A66\u90F5\u4EF6\u767C\u9001\u5931\u6557",
          "label_to": "\u6536\u4EF6\u4EBA",
          "label_subject": "\u4E3B\u984C",
          "placeholder_to": "user@example.com",
          "placeholder_subject": "\u6E2C\u8A66\u90F5\u4EF6\u4E3B\u984C"
        }
      },
      "telegram": {
        "title": "Telegram\u8A2D\u7F6E",
        "description": "\u914D\u7F6ETelegram\u6A5F\u5668\u4EBA\u529F\u80FD\uFF0C\u5BE6\u73FE\u7528\u6236\u901A\u77E5\u3001\u8CEC\u6236\u7D81\u5B9A\u3001\u6307\u4EE4\u4EA4\u4E92\u7B49\u81EA\u52D5\u5316\u670D\u52D9\u3002",
        "bot_token": {
          "title": "\u6A5F\u5668\u4EBA\u4EE4\u724C",
          "description": "\u8ACB\u8F38\u5165\u5F9EBotfather\u7372\u53D6\u7684\u4EE4\u724C\u3002",
          "placeholder": "0000000000:xxxxxxxxx_xxxxxxxxxxxxxxx"
        },
        "webhook_url": {
          "title": "Webhook \u57FA\u790E\u5730\u5740",
          "description": "\u9019\u88E1\u53EA\u586B\u5BEB\u57FA\u790E\u5730\u5740\uFF0C\u7CFB\u7D71\u6703\u81EA\u52D5\u62FC\u63A5 Telegram \u7684\u5B8C\u6574 Webhook \u56DE\u8ABF\u8DEF\u5F91\u3002\u7559\u7A7A\u6642\u9ED8\u8A8D\u4F7F\u7528\u7AD9\u9EDE\u7DB2\u5740\u3002",
          "docs": "\u67E5\u770B Telegram Webhook \u6587\u6A94",
          "placeholder": "https://example.com"
        },
        "webhook": {
          "title": "\u8A2D\u7F6EWebhook",
          "description": "\u8A2D\u7F6E\u6A5F\u5668\u4EBA\u7684webhook\uFF0C\u4E0D\u8A2D\u7F6E\u5C07\u7121\u6CD5\u6536\u5230Telegram\u901A\u77E5\u3002",
          "button": "\u4E00\u9375\u8A2D\u7F6E",
          "setting": "\u8A2D\u7F6E\u4E2D...",
          "success": "Webhook \u8A2D\u7F6E\u6210\u529F",
          "error": "Webhook \u8A2D\u7F6E\u5931\u6557",
          "target_default": "\u7576\u524D\u5C07\u4F7F\u7528\u7AD9\u9EDE\u7DB2\u5740\u4F5C\u70BA Webhook Base URL\u3002",
          "target_custom": "\u7576\u524D\u5C07\u4F7F\u7528\u81EA\u5B9A\u7FA9 Webhook Base URL\uFF1A{{url}}",
          "debug": {
            "title": "Webhook \u8ABF\u8A66\u4FE1\u606F",
            "success": "\u6210\u529F\u72C0\u614B",
            "url": "Webhook \u5730\u5740",
            "baseUrl": "\u57FA\u790E\u5730\u5740"
          }
        },
        "bot_enable": {
          "title": "\u555F\u7528Telegram\u7D81\u5B9A\u5F15\u5C0E",
          "description": "\u958B\u555F\u5F8C\u5C07\u5728\u7528\u6236\u7AEF\u986F\u793ATelegram\u7D81\u5B9A\u5F15\u5C0E\uFF0C\u5E6B\u52A9\u7528\u6236\u7D81\u5B9ATelegram\u8CEC\u6236\u4EE5\u63A5\u6536\u901A\u77E5\u3002"
        },
        "discuss_link": {
          "title": "\u7FA4\u7D44\u93C8\u63A5",
          "description": "\u586B\u5BEB\u5F8C\u5C07\u5728\u7528\u6236\u7AEF\u986F\u793A\u6216\u5728\u9700\u8981\u7684\u5730\u65B9\u4F7F\u7528\u3002",
          "placeholder": "https://t.me/xxxxxx"
        }
      },
      "app": {
        "title": "APP\u8A2D\u7F6E",
        "description": "\u7BA1\u7406\u79FB\u52D5\u61C9\u7528\u7A0B\u5E8F\u76F8\u95DC\u914D\u7F6E\uFF0C\u5305\u62ECAPI\u63A5\u53E3\u3001\u7248\u672C\u63A7\u5236\u3001\u63A8\u9001\u901A\u77E5\u7B49\u529F\u80FD\u8A2D\u7F6E\u3002",
        "common": {
          "placeholder": "\u8ACB\u8F38\u5165"
        },
        "windows": {
          "version": {
            "title": "Windows\u7248\u672C",
            "description": "Windows\u5BA2\u6236\u7AEF\u7576\u524D\u7248\u672C\u865F"
          },
          "download": {
            "title": "Windows\u4E0B\u8F09\u5730\u5740",
            "description": "Windows\u5BA2\u6236\u7AEF\u4E0B\u8F09\u93C8\u63A5"
          }
        },
        "macos": {
          "version": {
            "title": "macOS\u7248\u672C",
            "description": "macOS\u5BA2\u6236\u7AEF\u7576\u524D\u7248\u672C\u865F"
          },
          "download": {
            "title": "macOS\u4E0B\u8F09\u5730\u5740",
            "description": "macOS\u5BA2\u6236\u7AEF\u4E0B\u8F09\u93C8\u63A5"
          }
        },
        "android": {
          "version": {
            "title": "Android\u7248\u672C",
            "description": "Android\u5BA2\u6236\u7AEF\u7576\u524D\u7248\u672C\u865F"
          },
          "download": {
            "title": "Android\u4E0B\u8F09\u5730\u5740",
            "description": "Android\u5BA2\u6236\u7AEF\u4E0B\u8F09\u93C8\u63A5"
          }
        }
      },
      "common": {
        "saving": "\u4FDD\u5B58\u4E2D...",
        "save": "\u4FDD\u5B58",
        "save_success": "\u4FDD\u5B58\u6210\u529F",
        "save_error": "\u4FDD\u5B58\u5931\u6557",
        "reset": "\u91CD\u7F6E",
        "placeholder": "\u8ACB\u8F38\u5165",
        "autoSaved": "\u5DF2\u81EA\u52D5\u4FDD\u5B58",
        "saved_at": "{{seconds}} \u79D2\u524D\u5DF2\u4FDD\u5B58",
        "invalidJson": "JSON \u683C\u5F0F\u932F\u8AA4"
      },
      "invite": {
        "title": "\u9080\u8ACB&\u4F63\u91D1\u8A2D\u7F6E",
        "description": "\u9080\u8ACB\u8A3B\u518A\u3001\u4F63\u91D1\u76F8\u95DC\u8A2D\u7F6E\u3002",
        "invite_force": {
          "title": "\u958B\u555F\u5F37\u5236\u9080\u8ACB",
          "description": "\u958B\u555F\u5F8C\u53EA\u6709\u88AB\u9080\u8ACB\u7684\u7528\u6236\u624D\u53EF\u4EE5\u9032\u884C\u8A3B\u518A\u3002"
        },
        "invite_commission": {
          "title": "\u9080\u8ACB\u4F63\u91D1\u767E\u5206\u6BD4",
          "description": "\u9ED8\u8A8D\u5168\u5C40\u7684\u4F63\u91D1\u5206\u914D\u6BD4\u4F8B\uFF0C\u4F60\u53EF\u4EE5\u5728\u7528\u6236\u7BA1\u7406\u55AE\u7368\u914D\u7F6E\u55AE\u500B\u6BD4\u4F8B\u3002",
          "placeholder": "\u8ACB\u8F38\u5165\u4F63\u91D1\u767E\u5206\u6BD4"
        },
        "invite_gen_limit": {
          "title": "\u7528\u6236\u53EF\u5275\u5EFA\u9080\u8ACB\u78BC\u4E0A\u9650",
          "description": "\u7528\u6236\u53EF\u5275\u5EFA\u9080\u8ACB\u78BC\u4E0A\u9650",
          "placeholder": "\u8ACB\u8F38\u5165\u5275\u5EFA\u4E0A\u9650"
        },
        "invite_never_expire": {
          "title": "\u9080\u8ACB\u78BC\u6C38\u4E0D\u5931\u6548",
          "description": "\u958B\u555F\u5F8C\u9080\u8ACB\u78BC\u88AB\u4F7F\u7528\u5F8C\u5C07\u4E0D\u6703\u5931\u6548\uFF0C\u5426\u5247\u4F7F\u7528\u904E\u5F8C\u5373\u5931\u6548\u3002"
        },
        "commission_first_time": {
          "title": "\u4F63\u91D1\u50C5\u9996\u6B21\u767C\u653E",
          "description": "\u958B\u555F\u5F8C\u88AB\u9080\u8ACB\u4EBA\u9996\u6B21\u652F\u4ED8\u6642\u624D\u6703\u7522\u751F\u4F63\u91D1\uFF0C\u53EF\u4EE5\u5728\u7528\u6236\u7BA1\u7406\u5C0D\u7528\u6236\u9032\u884C\u55AE\u7368\u914D\u7F6E\u3002"
        },
        "commission_auto_check": {
          "title": "\u4F63\u91D1\u81EA\u52D5\u78BA\u8A8D",
          "description": "\u958B\u555F\u5F8C\u4F63\u91D1\u5C07\u6703\u5728\u8A02\u55AE\u5B8C\u62103\u65E5\u5F8C\u81EA\u52D5\u9032\u884C\u78BA\u8A8D\u3002"
        },
        "commission_withdraw_limit": {
          "title": "\u63D0\u73FE\u55AE\u7533\u8ACB\u9580\u6ABB(\u5143)",
          "description": "\u5C0F\u65BC\u9580\u6ABB\u91D1\u984D\u7684\u63D0\u73FE\u55AE\u5C07\u4E0D\u6703\u88AB\u63D0\u4EA4\u3002",
          "placeholder": "\u8ACB\u8F38\u5165\u63D0\u73FE\u9580\u6ABB"
        },
        "commission_withdraw_method": {
          "title": "\u63D0\u73FE\u65B9\u5F0F",
          "description": "\u53EF\u4EE5\u652F\u6301\u7684\u63D0\u73FE\u65B9\u5F0F\uFF0C\u591A\u500B\u7528\u9017\u865F\u5206\u9694\u3002",
          "placeholder": "\u8ACB\u8F38\u5165\u63D0\u73FE\u65B9\u5F0F\uFF0C\u591A\u500B\u7528\u9017\u865F\u5206\u9694"
        },
        "withdraw_close": {
          "title": "\u95DC\u9589\u63D0\u73FE",
          "description": "\u95DC\u9589\u5F8C\u5C07\u7981\u6B62\u7528\u6236\u7533\u8ACB\u63D0\u73FE\uFF0C\u4E14\u9080\u8ACB\u4F63\u91D1\u5C07\u6703\u76F4\u63A5\u9032\u5165\u7528\u6236\u9918\u984D\u3002"
        },
        "commission_distribution": {
          "title": "\u4E09\u7D1A\u5206\u92B7",
          "description": "\u958B\u555F\u5F8C\u5C07\u4F63\u91D1\u5C07\u6309\u7167\u8A2D\u7F6E\u76843\u6210\u6BD4\u4F8B\u9032\u884C\u5206\u6210\uFF0C\u4E09\u6210\u6BD4\u4F8B\u5408\u8A08\u8ACB\u4E0D\u8981\u5927\u65BC100%\u3002",
          "l1": "\u4E00\u7D1A\u9080\u8ACB\u4EBA\u6BD4\u4F8B",
          "l2": "\u4E8C\u7D1A\u9080\u8ACB\u4EBA\u6BD4\u4F8B",
          "l3": "\u4E09\u7D1A\u9080\u8ACB\u4EBA\u6BD4\u4F8B",
          "placeholder": "\u8ACB\u8F38\u5165\u6BD4\u4F8B\uFF0C\u5982\uFF1A50"
        },
        "distribution_total": "\u7576\u524D\u5206\u92B7\u6BD4\u4F8B\u5408\u8A08\uFF1A{{total}}%\uFF08\u4E0D\u61C9\u8D85\u904E 100%\uFF09",
        "saving": "\u4FDD\u5B58\u4E2D..."
      },
      "server": {
        "title": "\u7BC0\u9EDE\u914D\u7F6E",
        "description": "\u914D\u7F6E\u7BC0\u9EDE\u901A\u4FE1\u548C\u540C\u6B65\u8A2D\u7F6E\uFF0C\u5305\u62EC\u901A\u4FE1\u5BC6\u9470\u3001\u8F2A\u8A62\u9593\u9694\u3001\u8CA0\u8F09\u5747\u8861\u7B49\u9AD8\u7D1A\u9078\u9805\u3002",
        "server_token": {
          "title": "\u901A\u8A0A\u5BC6\u9470",
          "description": "Fboard\u8207\u7BC0\u9EDE\u901A\u8A0A\u7684\u5BC6\u9470\uFF0C\u4EE5\u4FBF\u6578\u64DA\u4E0D\u6703\u88AB\u4ED6\u4EBA\u7372\u53D6\u3002",
          "placeholder": "\u8ACB\u8F38\u5165\u901A\u8A0A\u5BC6\u9470",
          "generate_tooltip": "\u9EDE\u64CA\u751F\u6210\u96A8\u6A5F\u901A\u4FE1\u5BC6\u9470"
        },
        "server_pull_interval": {
          "title": "\u7BC0\u9EDE\u62C9\u53D6\u52D5\u4F5C\u8F2A\u8A62\u9593\u9694",
          "description": "\u7BC0\u9EDE\u5F9E\u9762\u677F\u7372\u53D6\u6578\u64DA\u7684\u9593\u9694\u983B\u7387\u3002",
          "placeholder": "\u8ACB\u8F38\u5165\u62C9\u53D6\u9593\u9694"
        },
        "server_push_interval": {
          "title": "\u7BC0\u9EDE\u63A8\u9001\u52D5\u4F5C\u8F2A\u8A62\u9593\u9694",
          "description": "\u7BC0\u9EDE\u63A8\u9001\u6578\u64DA\u5230\u9762\u677F\u7684\u9593\u9694\u983B\u7387\u3002",
          "placeholder": "\u8ACB\u8F38\u5165\u63A8\u9001\u9593\u9694"
        },
        "device_limit_mode": {
          "title": "\u8A2D\u5099\u9650\u5236\u6A21\u5F0F",
          "description": "\u5BEC\u9B06\u6A21\u5F0F\u4E0B\uFF0C\u540C\u4E00IP\u5730\u5740\u4F7F\u7528\u591A\u500B\u7BC0\u9EDE\u53EA\u7D71\u8A08\u70BA\u4E00\u500B\u8A2D\u5099\u3002",
          "strict": "\u56B4\u683C\u6A21\u5F0F",
          "relaxed": "\u5BEC\u9B06\u6A21\u5F0F",
          "placeholder": "\u8ACB\u9078\u64C7\u8A2D\u5099\u9650\u5236\u6A21\u5F0F"
        },
        "server_ws_enable": {
          "title": "\u555F\u7528 WebSocket \u901A\u4FE1",
          "description": "\u958B\u555F\u5F8C\u7BC0\u9EDE\u5C07\u901A\u904E WebSocket \u8207\u9762\u677F\u9032\u884C\u5BE6\u6642\u901A\u4FE1\uFF0C\u5EF6\u9072\u66F4\u4F4E\u3001\u63A8\u9001\u66F4\u53CA\u6642\u3002",
          "supported_clients": "\u76EE\u524D\u652F\u6301 WebSocket \u901A\u4FE1\u7684\u7BC0\u9EDE\u7AEF\uFF1AFboard-Node"
        },
        "server_ws_url": {
          "title": "WebSocket \u5730\u5740",
          "description": "\u7BC0\u9EDE\u9023\u63A5\u9762\u677F\u7684 WebSocket \u5730\u5740\uFF0C\u7559\u7A7A\u5247\u81EA\u52D5\u4F7F\u7528\u7AD9\u9EDE\u7DB2\u5740\u3002",
          "placeholder": "\u7559\u7A7A\u5247\u4F7F\u7528\u7AD9\u9EDE\u7DB2\u5740"
        },
        "server_ws_log_enable": {
          "title": "WebSocket \u8ABF\u8A66\u65E5\u8A8C",
          "description": "\u958B\u555F\u5F8C\u8A18\u9304\u7BC0\u9EDE/\u6A5F\u5668\u9023\u63A5\u3001\u5168\u91CF\u540C\u6B65\u3001\u63A8\u9001\u7B49 [WS] \u65E5\u8A8C\u3002\u9ED8\u8A8D\u95DC\u9589\uFF0C\u907F\u514D\u7BC0\u9EDE\u8F03\u591A\u6642\u5237\u5C4F\uFF1B\u7570\u5E38\u544A\u8B66\u59CB\u7D42\u6703\u5BEB\u5165\u3002\u4FEE\u6539\u5F8C\u6578\u79D2\u5167\u751F\u6548\uFF0C\u7121\u9700\u91CD\u555F\u3002"
        },
        "node_install_script_url": {
          "title": "\u7BC0\u9EDE\u5B89\u88DD\u8173\u672C\u5730\u5740",
          "description": "\u81EA\u5B9A\u7FA9\u7BC0\u9EDE\u5B89\u88DD\u8173\u672C\u7684 URL \u5730\u5740\uFF0C\u7559\u7A7A\u5247\u4F7F\u7528\u9ED8\u8A8D\u7684 GitHub \u5730\u5740\u3002",
          "placeholder": "\u7559\u7A7A\u5247\u4F7F\u7528\u9ED8\u8A8D\u5730\u5740"
        },
        "utls_fingerprints": {
          "title": "uTLS \u6307\u7D0B\u5217\u8868",
          "description": "\u7DE8\u8F2F\u7BC0\u9EDE\u6642\u300CuTLS \u8A2D\u7F6E \u2192 \u6307\u7D0B\u300D\u7684\u5177\u9AD4\u6307\u7D0B\u53EF\u9078\u9805\u3002\u652F\u6301 chrome / firefox / safari / ios / android / edge / qq \u7B49\uFF0C\u4E5F\u53EF\u6DFB\u52A0\u5BA2\u6236\u7AEF\u652F\u6301\u7684\u5176\u5B83\u6307\u7D0B\u540D\u3002random\uFF08\u8A02\u95B1\u6642\u96A8\u6A5F\uFF09\u8207 randomized\uFF08\u5BA2\u6236\u7AEF\u5167\u96A8\u6A5F\uFF09\u7531\u7CFB\u7D71\u56FA\u5B9A\u63D0\u4F9B\uFF0C\u7121\u9700\u5728\u6B64\u914D\u7F6E\u3002",
          "placeholder": "\u8F38\u5165\u6307\u7D0B\u540D\u7A31\u5F8C\u56DE\u8ECA\u6DFB\u52A0\uFF0C\u5982 chrome"
        },
        "saving": "\u4FDD\u5B58\u4E2D...",
        "manage": {
          "title": "\u7BC0\u9EDE\u7BA1\u7406",
          "description": "\u7BA1\u7406\u6240\u6709\u7BC0\u9EDE\uFF0C\u5305\u62EC\u6DFB\u52A0\u3001\u522A\u9664\u3001\u7DE8\u8F2F\u7B49\u64CD\u4F5C\u3002"
        }
      },
      "subscribe_template": {
        "title": "\u8A02\u95B1\u6A21\u677F",
        "description": "\u914D\u7F6E\u5404\u500B\u5BA2\u6236\u7AEF\u7684\u8A02\u95B1\u6A21\u677F\uFF08\u6309\u9700\u52A0\u8F09\uFF0C\u9700\u624B\u52D5\u4FDD\u5B58\uFF09",
        "unsaved_hint": "\u7576\u524D\u6A21\u677F\u6709\u672A\u4FDD\u5B58\u7684\u4FEE\u6539\uFF0C\u8ACB\u9EDE\u64CA\u4FDD\u5B58\u3002",
        "singbox": {
          "title": "Sing-box \u8A02\u95B1\u6A21\u677F",
          "description": "\u914D\u7F6E Sing-box \u7684\u8A02\u95B1\u6A21\u677F\u683C\u5F0F"
        },
        "clash": {
          "title": "Clash \u8A02\u95B1\u6A21\u677F",
          "description": "\u914D\u7F6E Clash \u7684\u8A02\u95B1\u6A21\u677F\u683C\u5F0F"
        },
        "clashmeta": {
          "title": "Clash Meta \u8A02\u95B1\u6A21\u677F",
          "description": "\u914D\u7F6E Clash Meta \u7684\u8A02\u95B1\u6A21\u677F\u683C\u5F0F"
        },
        "stash": {
          "title": "Stash \u8A02\u95B1\u6A21\u677F",
          "description": "\u914D\u7F6E Stash \u7684\u8A02\u95B1\u6A21\u677F\u683C\u5F0F"
        },
        "surge": {
          "title": "Surge \u914D\u7F6E\u6A21\u677F",
          "description": "\u914D\u7F6E Surge \u8A02\u95B1\u6A21\u677F\uFF0C\u652F\u6301 Surge \u914D\u7F6E\u6587\u4EF6\u683C\u5F0F"
        },
        "surfboard": {
          "title": "Surfboard \u914D\u7F6E\u6A21\u7248",
          "description": "\u914D\u984D Surfboard \u8A02\u95B1\u6A21\u7248"
        }
      },
      "email_template": {
        "title": "\u90F5\u4EF6\u6A21\u677F",
        "description": "\u81EA\u5B9A\u7FA9\u7CFB\u7D71\u767C\u9001\u7684\u5404\u985E\u90F5\u4EF6\u5167\u5BB9\u6A21\u677F",
        "customized": "\u5DF2\u81EA\u5B9A\u7FA9",
        "subject": "\u90F5\u4EF6\u4E3B\u984C",
        "subject_placeholder": "\u8F38\u5165\u90F5\u4EF6\u4E3B\u984C\uFF0C\u652F\u6301 {{name}} \u7B49\u4F54\u4F4D\u7B26",
        "content": "\u6A21\u677F\u5167\u5BB9 (HTML)",
        "preview": "\u5BE6\u6642\u9810\u89BD",
        "override_hint": "\u4FEE\u6539\u4E26\u4FDD\u5B58\u5F8C\u5C07\u8986\u84CB\u7CFB\u7D71\u9ED8\u8A8D\u6A21\u677F\u3002\u9EDE\u64CA\u300C\u6062\u5FA9\u9ED8\u8A8D\u300D\u53EF\u96A8\u6642\u9084\u539F\u70BA\u7576\u524D\u4E3B\u984C\u7684\u9ED8\u8A8D\u6A21\u677F\u3002",
        "placeholders": "\u53EF\u7528\u4F54\u4F4D\u7B26",
        "var_name": "\u8B8A\u91CF",
        "var_desc": "\u8AAA\u660E",
        "var_sample": "\u793A\u4F8B\u503C",
        "required": "\u5FC5\u586B",
        "insert": "\u63D2\u5165",
        "placeholder_hint": "* \u6A19\u8A18\u70BA\u5FC5\u9808\u5305\u542B\u7684\u4F54\u4F4D\u7B26\uFF0C\u9EDE\u64CA\u53EF\u63D2\u5165\u5230\u5167\u5BB9\u672B\u5C3E",
        "click_to_insert": "\u9EDE\u64CA\u63D2\u5165",
        "save": "\u4FDD\u5B58",
        "save_success": "\u6A21\u677F\u4FDD\u5B58\u6210\u529F",
        "save_before_test": "\u8ACB\u5148\u4FDD\u5B58\u4FEE\u6539\u5F8C\u518D\u767C\u9001\u6E2C\u8A66",
        "send_test": "\u767C\u9001\u6E2C\u8A66",
        "test_dialog_title": "\u767C\u9001\u6E2C\u8A66\u90F5\u4EF6",
        "test_dialog_description": "\u8F38\u5165\u6536\u4EF6\u90F5\u7BB1\uFF0C\u7559\u7A7A\u5C07\u767C\u9001\u5230\u7576\u524D\u7BA1\u7406\u54E1\u90F5\u7BB1",
        "test_email_placeholder": "\u6536\u4EF6\u90F5\u7BB1\uFF08\u7559\u7A7A\u4F7F\u7528\u7576\u524D\u8CEC\u865F\uFF09",
        "sending": "\u767C\u9001\u4E2D...",
        "test_success": "\u6E2C\u8A66\u90F5\u4EF6\u5DF2\u767C\u9001",
        "reset": "\u6062\u5FA9\u9ED8\u8A8D",
        "reset_title": "\u6062\u5FA9\u9ED8\u8A8D\u6A21\u677F",
        "reset_description": "\u78BA\u5B9A\u8981\u6062\u5FA9\u6B64\u6A21\u677F\u70BA\u9ED8\u8A8D\u5167\u5BB9\u55CE\uFF1F\u81EA\u5B9A\u7FA9\u7684\u5167\u5BB9\u5C07\u88AB\u522A\u9664\u3002",
        "reset_confirm": "\u78BA\u5B9A\u6062\u5FA9",
        "reset_success": "\u5DF2\u6062\u5FA9\u9ED8\u8A8D\u6A21\u677F",
        "unsaved": "\u6709\u672A\u4FDD\u5B58\u7684\u4FEE\u6539",
        "discard_title": "\u672A\u4FDD\u5B58\u7684\u4FEE\u6539",
        "discard_description": "\u7576\u524D\u6A21\u677F\u6709\u672A\u4FDD\u5B58\u7684\u4FEE\u6539\uFF0C\u5207\u63DB\u6A19\u7C64\u9801\u5C07\u4E1F\u5931\u9019\u4E9B\u4FEE\u6539\u3002",
        "discard_confirm": "\u4E1F\u68C4\u4FEE\u6539",
        "cancel": "\u53D6\u6D88"
      }
    },
    "notice": {
      "title": "\u516C\u544A\u7BA1\u7406",
      "description": "\u5728\u9019\u88E1\u53EF\u4EE5\u914D\u7F6E\u516C\u544A\uFF0C\u5305\u62EC\u6DFB\u52A0\u3001\u522A\u9664\u3001\u7DE8\u8F2F\u7B49\u64CD\u4F5C\u3002",
      "table": {
        "columns": {
          "id": "ID",
          "show": "\u986F\u793A\u72C0\u614B",
          "title": "\u6A19\u984C",
          "actions": "\u64CD\u4F5C"
        },
        "toolbar": {
          "search": "\u641C\u7D22\u516C\u544A\u6A19\u984C...",
          "reset": "\u91CD\u7F6E",
          "sort": {
            "edit": "\u7DE8\u8F2F\u6392\u5E8F",
            "save": "\u4FDD\u5B58\u6392\u5E8F"
          }
        },
        "actions": {
          "edit": "\u7DE8\u8F2F",
          "delete": {
            "title": "\u522A\u9664\u78BA\u8A8D",
            "description": "\u78BA\u5B9A\u8981\u522A\u9664\u8A72\u689D\u516C\u544A\u55CE\uFF1F\u6B64\u64CD\u4F5C\u7121\u6CD5\u64A4\u92B7\u3002",
            "success": "\u522A\u9664\u6210\u529F"
          }
        }
      },
      "form": {
        "add": {
          "title": "\u6DFB\u52A0\u516C\u544A",
          "button": "\u6DFB\u52A0\u516C\u544A"
        },
        "edit": {
          "title": "\u7DE8\u8F2F\u516C\u544A"
        },
        "fields": {
          "title": {
            "label": "\u6A19\u984C",
            "placeholder": "\u8ACB\u8F38\u5165\u516C\u544A\u6A19\u984C"
          },
          "content": {
            "label": "\u516C\u544A\u5167\u5BB9"
          },
          "img_url": {
            "label": "\u516C\u544A\u80CC\u666F",
            "placeholder": "\u8ACB\u8F38\u5165\u516C\u544A\u80CC\u666F\u5716\u7247URL"
          },
          "show": {
            "label": "\u986F\u793A"
          },
          "tags": {
            "label": "\u7BC0\u9EDE\u6A19\u7C64",
            "placeholder": "\u8F38\u5165\u5F8C\u56DE\u8ECA\u6DFB\u52A0\u6A19\u7C64"
          }
        },
        "buttons": {
          "cancel": "\u53D6\u6D88",
          "submit": "\u63D0\u4EA4",
          "success": "\u63D0\u4EA4\u6210\u529F"
        }
      },
      "messages": {
        "loadError": "\u52A0\u8F09\u516C\u544A\u5931\u6557"
      }
    },
    "group": {
      "title": "\u6B0A\u9650\u7D44\u7BA1\u7406",
      "description": "\u7BA1\u7406\u6240\u6709\u6B0A\u9650\u7D44\uFF0C\u5305\u62EC\u6DFB\u52A0\u3001\u522A\u9664\u3001\u7DE8\u8F2F\u7B49\u64CD\u4F5C\u3002",
      "columns": {
        "id": "\u7D44ID",
        "name": "\u7D44\u540D\u7A31",
        "usersCount": "\u7528\u6236\u6578\u91CF",
        "serverCount": "\u7BC0\u9EDE\u6578\u91CF",
        "actions": "\u64CD\u4F5C"
      },
      "form": {
        "add": "\u6DFB\u52A0\u6B0A\u9650\u7D44",
        "edit": "\u7DE8\u8F2F\u6B0A\u9650\u7D44",
        "create": "\u5275\u5EFA\u6B0A\u9650\u7D44",
        "update": "\u66F4\u65B0",
        "name": "\u7D44\u540D\u7A31",
        "namePlaceholder": "\u8ACB\u8F38\u5165\u6B0A\u9650\u7D44\u540D\u7A31",
        "nameDescription": "\u6B0A\u9650\u7D44\u540D\u7A31\u7528\u65BC\u6A19\u8B58\u4E0D\u540C\u7684\u7528\u6236\u7D44\uFF0C\u5EFA\u8B70\u4F7F\u7528\u6709\u610F\u7FA9\u7684\u540D\u7A31\u3002",
        "cancel": "\u53D6\u6D88",
        "editDescription": "\u4FEE\u6539\u6B0A\u9650\u7D44\u4FE1\u606F\uFF0C\u66F4\u65B0\u5F8C\u6703\u7ACB\u5373\u751F\u6548\u3002",
        "createDescription": "\u5275\u5EFA\u65B0\u7684\u6B0A\u9650\u7D44\uFF0C\u53EF\u4EE5\u70BA\u4E0D\u540C\u7684\u7528\u6236\u5206\u914D\u4E0D\u540C\u7684\u6B0A\u9650\u3002"
      },
      "toolbar": {
        "searchPlaceholder": "\u641C\u7D22\u6B0A\u9650\u7D44...",
        "reset": "\u91CD\u7F6E"
      },
      "messages": {
        "deleteConfirm": "\u78BA\u8A8D\u522A\u9664",
        "deleteDescription": "\u6B64\u64CD\u4F5C\u5C07\u6C38\u4E45\u522A\u9664\u8A72\u6B0A\u9650\u7D44\uFF0C\u522A\u9664\u5F8C\u7121\u6CD5\u6062\u5FA9\u3002\u78BA\u5B9A\u8981\u7E7C\u7E8C\u55CE\uFF1F",
        "deleteButton": "\u522A\u9664",
        "createSuccess": "\u5275\u5EFA\u6210\u529F",
        "updateSuccess": "\u66F4\u65B0\u6210\u529F",
        "nameValidation": {
          "min": "\u7D44\u540D\u81F3\u5C11\u9700\u89812\u500B\u5B57\u7B26",
          "max": "\u7D44\u540D\u4E0D\u80FD\u8D85\u904E50\u500B\u5B57\u7B26",
          "pattern": "\u7D44\u540D\u53EA\u80FD\u5305\u542B\u5B57\u6BCD\u3001\u6578\u5B57\u3001\u4E2D\u6587\u3001\u4E0B\u5283\u7DDA\u548C\u9023\u5B57\u7B26"
        }
      }
    },
    "auth": {
      "signIn": {
        "title": "\u767B\u9304",
        "description": "\u8ACB\u8F38\u5165\u60A8\u7684\u90F5\u7BB1\u548C\u5BC6\u78BC\u767B\u9304\u7CFB\u7D71",
        "email": "\u90F5\u7BB1\u5730\u5740",
        "emailPlaceholder": "name@example.com",
        "password": "\u5BC6\u78BC",
        "passwordPlaceholder": "\u8ACB\u8F38\u5165\u5BC6\u78BC",
        "forgotPassword": "\u5FD8\u8A18\u5BC6\u78BC\uFF1F",
        "submit": "\u767B\u9304",
        "rememberMe": "\u8A18\u4F4F\u6211",
        "resetPassword": {
          "title": "\u91CD\u7F6E\u5BC6\u78BC",
          "description": "\u5728\u7AD9\u9EDE\u76EE\u9304\u4E0B\u57F7\u884C\u4EE5\u4E0B\u547D\u4EE4\u627E\u56DE\u5BC6\u78BC",
          "command": "php artisan reset:password \u7BA1\u7406\u54E1\u90F5\u7BB1"
        },
        "validation": {
          "emailRequired": "\u8ACB\u8F38\u5165\u90F5\u7BB1\u5730\u5740",
          "emailInvalid": "\u90F5\u7BB1\u5730\u5740\u683C\u5F0F\u4E0D\u6B63\u78BA",
          "passwordRequired": "\u8ACB\u8F38\u5165\u5BC6\u78BC",
          "passwordLength": "\u5BC6\u78BC\u9577\u5EA6\u81F3\u5C11\u70BA7\u500B\u5B57\u7B26"
        }
      }
    },
    "traffic": {
      "trafficRecord": {
        "title": "\u6D41\u91CF\u4F7F\u7528\u8A18\u9304",
        "time": "\u6642\u9593",
        "upload": "\u4E0A\u884C\u6D41\u91CF",
        "download": "\u4E0B\u884C\u6D41\u91CF",
        "rate": "\u500D\u7387",
        "total": "\u7E3D\u8A08",
        "noRecords": "\u66AB\u7121\u8A18\u9304",
        "perPage": "\u6BCF\u9801\u986F\u793A",
        "records": "\u689D\u8A18\u9304",
        "page": "\u7B2C {{current}} / {{total}} \u9801",
        "multiplier": "{{value}}x"
      }
    },
    "payment": {
      "title": "\u652F\u4ED8\u914D\u7F6E",
      "description": "\u5728\u9019\u88E1\u53EF\u4EE5\u914D\u7F6E\u652F\u4ED8\u65B9\u5F0F\uFF0C\u5305\u62EC\u652F\u4ED8\u5BF6\u3001\u5FAE\u4FE1\u7B49\u3002",
      "table": {
        "columns": {
          "id": "ID",
          "enable": "\u555F\u7528",
          "name": "\u986F\u793A\u540D\u7A31",
          "payment": "\u652F\u4ED8\u63A5\u53E3",
          "notify_url": "\u901A\u77E5\u5730\u5740",
          "notify_url_tooltip": "\u652F\u4ED8\u7DB2\u95DC\u5C07\u6703\u628A\u6578\u64DA\u901A\u77E5\u5230\u672C\u5730\u5740\uFF0C\u8ACB\u901A\u904E\u9632\u706B\u7246\u653E\u884C\u672C\u5730\u5740\u3002",
          "actions": "\u64CD\u4F5C"
        },
        "actions": {
          "edit": "\u7DE8\u8F2F",
          "copy": "\u8907\u88FD",
          "copy_success": "\u8907\u88FD\u6210\u529F",
          "delete": {
            "title": "\u522A\u9664\u78BA\u8A8D",
            "description": "\u78BA\u5B9A\u8981\u522A\u9664\u8A72\u652F\u4ED8\u65B9\u5F0F\u55CE\uFF1F\u6B64\u64CD\u4F5C\u7121\u6CD5\u64A4\u92B7\u3002",
            "success": "\u522A\u9664\u6210\u529F"
          }
        },
        "toolbar": {
          "search": "\u641C\u7D22\u652F\u4ED8\u65B9\u5F0F...",
          "reset": "\u91CD\u7F6E",
          "sort": {
            "hint": "\u62D6\u62FD\u652F\u4ED8\u65B9\u5F0F\u9032\u884C\u6392\u5E8F\uFF0C\u5B8C\u6210\u5F8C\u9EDE\u64CA\u4FDD\u5B58",
            "save": "\u4FDD\u5B58\u6392\u5E8F",
            "edit": "\u7DE8\u8F2F\u6392\u5E8F"
          }
        }
      },
      "form": {
        "add": {
          "button": "\u6DFB\u52A0\u652F\u4ED8\u65B9\u5F0F",
          "title": "\u6DFB\u52A0\u652F\u4ED8\u65B9\u5F0F"
        },
        "edit": {
          "title": "\u7DE8\u8F2F\u652F\u4ED8\u65B9\u5F0F"
        },
        "fields": {
          "name": {
            "label": "\u986F\u793A\u540D\u7A31",
            "placeholder": "\u8ACB\u8F38\u5165\u652F\u4ED8\u540D\u7A31",
            "description": "\u7528\u65BC\u524D\u7AEF\u986F\u793A"
          },
          "icon": {
            "label": "\u5716\u6A19URL",
            "placeholder": "https://example.com/icon.svg",
            "description": "\u7528\u65BC\u524D\u7AEF\u986F\u793A\u7684\u5716\u6A19\u5730\u5740"
          },
          "notify_domain": {
            "label": "\u901A\u77E5\u57DF\u540D",
            "placeholder": "https://example.com",
            "description": "\u7DB2\u95DC\u901A\u77E5\u5C07\u767C\u9001\u5230\u8A72\u57DF\u540D"
          },
          "handling_fee_percent": {
            "label": "\u767E\u5206\u6BD4\u624B\u7E8C\u8CBB(%)",
            "placeholder": "0-100"
          },
          "handling_fee_fixed": {
            "label": "\u56FA\u5B9A\u624B\u7E8C\u8CBB",
            "placeholder": "0"
          },
          "payment": {
            "label": "\u652F\u4ED8\u63A5\u53E3",
            "placeholder": "\u8ACB\u9078\u64C7\u652F\u4ED8\u63A5\u53E3",
            "description": "\u9078\u64C7\u8981\u4F7F\u7528\u7684\u652F\u4ED8\u63A5\u53E3"
          }
        },
        "validation": {
          "name": {
            "min": "\u540D\u7A31\u81F3\u5C11\u9700\u89812\u500B\u5B57\u7B26",
            "max": "\u540D\u7A31\u4E0D\u80FD\u8D85\u904E30\u500B\u5B57\u7B26"
          },
          "notify_domain": {
            "url": "\u8ACB\u8F38\u5165\u6709\u6548\u7684URL"
          },
          "payment": {
            "required": "\u8ACB\u9078\u64C7\u652F\u4ED8\u63A5\u53E3"
          }
        },
        "buttons": {
          "cancel": "\u53D6\u6D88",
          "submit": "\u63D0\u4EA4"
        },
        "sections": {
          "payment_config": "\u652F\u4ED8\u914D\u7F6E"
        },
        "messages": {
          "success": "\u4FDD\u5B58\u6210\u529F"
        },
        "config": {
          "title": "\u652F\u4ED8\u914D\u7F6E",
          "noConfig": "\u8A72\u652F\u4ED8\u65B9\u5F0F\u66AB\u7121\u914D\u7F6E\u9805"
        }
      }
    },
    "server": {
      "manage": {
        "title": "\u7BC0\u9EDE\u7BA1\u7406",
        "description": "\u7BA1\u7406\u6240\u6709\u7BC0\u9EDE\uFF0C\u5305\u62EC\u6DFB\u52A0\u3001\u522A\u9664\u3001\u7DE8\u8F2F\u7B49\u64CD\u4F5C\u3002",
        "filtered_by_server": "\u7576\u524D\u6B63\u5728\u67E5\u770B\u670D\u52D9\u5668 {{server}} (SID:{{id}}) \u4E0B\u7684\u7BC0\u9EDE",
        "filtered_by_server_description": "\u5728\u9019\u88E1\u65B0\u589E\u7BC0\u9EDE\u6642\uFF0C\u53EF\u4EE5\u76F4\u63A5\u8907\u7528\u7576\u524D\u670D\u52D9\u5668\u4F5C\u70BA\u90E8\u7F72\u76EE\u6A19\u3002",
        "add_node_to_server": "\u65B0\u589E\u7BC0\u9EDE\u5230\u6B64\u670D\u52D9\u5668",
        "clear_server_filter": "\u6E05\u9664\u670D\u52D9\u5668\u7BE9\u9078"
      },
      "columns": {
        "sort": "\u6392\u5E8F",
        "nodeId": "\u7BC0\u9EDEID",
        "show": "\u986F\u96B1",
        "node": "\u7BC0\u9EDE",
        "address": "\u5730\u5740",
        "onlineUsers": {
          "title": "\u5728\u7DDA\u4EBA\u6578",
          "tooltip": "\u5728\u7DDA\u4EBA\u6578\u6839\u64DA\u670D\u52D9\u7AEF\u4E0A\u5831\u983B\u7387\u800C\u5B9A",
          "sort_tip": "\u6309\u5728\u7DDA\u4EBA\u6578\u6392\u5E8F"
        },
        "rate": {
          "title": "\u500D\u7387",
          "tooltip": "\u6D41\u91CF\u6263\u8CBB\u500D\u7387"
        },
        "traffic": {
          "title": "\u6D41\u91CF\u4F7F\u7528",
          "tooltip": "\u7BC0\u9EDE\u6D41\u91CF\u4F7F\u7528\u60C5\u6CC1\uFF0C\u986F\u793A\u5DF2\u7528\u6D41\u91CF\u548C\u9650\u5236",
          "total": "\u7E3D\u6D41\u91CF",
          "used": "\u5DF2\u7528",
          "percentage": "\u4F7F\u7528\u7387"
        },
        "groups": {
          "title": "\u6B0A\u9650\u7D44",
          "tooltip": "\u53EF\u8A02\u95B1\u5230\u8A72\u7BC0\u9EDE\u7684\u6B0A\u9650\u7D44",
          "empty": "--"
        },
        "loadStatus": {
          "title": "\u8CA0\u8F09\u72C0\u614B",
          "tooltip": "\u670D\u52D9\u5668\u8CC7\u6E90\u4F7F\u7528\u60C5\u6CC1",
          "noData": "\u66AB\u7121\u6578\u64DA",
          "details": "\u7CFB\u7D71\u8CA0\u8F09\u8A73\u60C5",
          "cpu": "CPU \u4F7F\u7528\u7387",
          "memory": "\u5167\u5B58\u4F7F\u7528",
          "swap": "\u4EA4\u63DB\u5340",
          "disk": "\u78C1\u76E4\u4F7F\u7528",
          "lastUpdate": "\u6700\u5F8C\u66F4\u65B0",
          "metrics": {
            "title": "\u904B\u884C\u6307\u6A19",
            "uptime": "\u904B\u884C\u6642\u9577",
            "conns": "\u5BE6\u6642/\u7E3D\u9023\u63A5",
            "speed": "\u5BE6\u6642\u901F\u7387",
            "api": "API \u72C0\u614B",
            "kernel": "\u5167\u6838\u72C0\u614B",
            "gc": "GC \u66AB\u505C",
            "limit": "\u9650\u901F\u7528\u6236",
            "ws": "WebSocket",
            "goroutines": "\u4F75\u767C\u5354\u7A0B",
            "load": "\u7CFB\u7D71\u8CA0\u8F09",
            "users": "\u5728\u7DDA\u7528\u6236"
          }
        },
        "version": "\u7248\u672C",
        "customId": "\u81EA\u5B9A\u7FA9ID",
        "originalId": "\u539F\u59CBID",
        "type": "\u985E\u578B",
        "actions": "\u64CD\u4F5C",
        "copyAddress": "\u8907\u88FD\u9023\u63A5\u5730\u5740",
        "internalPort": "\u5167\u90E8\u7AEF\u53E3",
        "deployment": {
          "title": "\u90E8\u7F72\u65B9\u5F0F",
          "tooltip": "\u67E5\u770B\u7BC0\u9EDE\u662F\u7368\u7ACB\u90E8\u7F72\uFF0C\u9084\u662F\u7531\u67D0\u81FA\u670D\u52D9\u5668\u8A17\u7BA1\uFF0C\u4E26\u53EF\u76F4\u63A5\u5728\u5217\u8868\u4E2D\u8ABF\u6574\u3002",
          "standalone": "\u7368\u7ACB\u90E8\u7F72",
          "standalone_row_hint": "\u672A\u7D81\u5B9A\u670D\u52D9\u5668",
          "standalone_description": "\u8A72\u7BC0\u9EDE\u4E0D\u4F9D\u8CF4\u670D\u52D9\u5668\u8A17\u7BA1\uFF0C\u9069\u7528\u65BC\u55AE\u7BC0\u9EDE\u7368\u7ACB\u90E8\u7F72\u3002",
          "online": "\u5728\u7DDA",
          "offline": "\u96E2\u7DDA",
          "inactive": "\u670D\u52D9\u5668\u672A\u6FC0\u6D3B",
          "disabled": "\u7BC0\u9EDE\u505C\u7528",
          "enabled": "\u5728\u670D\u52D9\u5668\u4E0A\u555F\u7528",
          "enabled_description": "\u50C5\u5DF2\u555F\u7528\u7684\u7BC0\u9EDE\u6703\u7531\u6240\u9078\u670D\u52D9\u5668\u62C9\u8D77\u4E26\u540C\u6B65\u3002",
          "enabled_standalone_description": "\u7368\u7ACB\u90E8\u7F72\u7BC0\u9EDE\u7121\u9700\u8A2D\u7F6E\u670D\u52D9\u5668\u555F\u7528\u72C0\u614B\u3002",
          "bind_success": "\u5DF2\u8A17\u7BA1\u5230 {{server}}",
          "standalone_success": "\u5DF2\u5207\u63DB\u70BA\u7368\u7ACB\u90E8\u7F72",
          "update_success": "\u90E8\u7F72\u72C0\u614B\u5DF2\u66F4\u65B0",
          "update_error": "\u90E8\u7F72\u72C0\u614B\u66F4\u65B0\u5931\u6557"
        },
        "status": {
          "0": "\u672A\u904B\u884C",
          "1": "\u7121\u4EBA\u4F7F\u7528\u6216\u7570\u5E38",
          "2": "\u904B\u884C\u6B63\u5E38"
        },
        "childNode": "\u5B50\u7BC0\u9EDE",
        "actions_dropdown": {
          "edit": "\u7DE8\u8F2F",
          "copy": "\u8907\u88FD",
          "reset_traffic": {
            "title": "\u78BA\u8A8D\u91CD\u7F6E\u6D41\u91CF",
            "description": "\u6B64\u64CD\u4F5C\u5C07\u6E05\u96F6\u8A72\u7BC0\u9EDE\u7684\u4E0A\u50B3\u548C\u4E0B\u8F09\u6D41\u91CF\uFF0C\u4E26\u89E3\u9664\u7981\u7528\u72C0\u614B\u3002\u78BA\u5B9A\u8981\u7E7C\u7E8C\u55CE\uFF1F",
            "confirm": "\u91CD\u7F6E\u6D41\u91CF"
          },
          "reset_traffic_success": "\u6D41\u91CF\u91CD\u7F6E\u6210\u529F",
          "delete": {
            "title": "\u78BA\u8A8D\u522A\u9664",
            "description": "\u6B64\u64CD\u4F5C\u5C07\u6C38\u4E45\u522A\u9664\u8A72\u7BC0\u9EDE\uFF0C\u522A\u9664\u5F8C\u7121\u6CD5\u6062\u5FA9\u3002\u78BA\u5B9A\u8981\u7E7C\u7E8C\u55CE\uFF1F",
            "confirm": "\u522A\u9664"
          },
          "copy_success": "\u8907\u88FD\u6210\u529F",
          "delete_success": "\u522A\u9664\u6210\u529F"
        }
      },
      "toolbar": {
        "search": "\u641C\u7D22\u7BC0\u9EDE...",
        "type": "\u985E\u578B",
        "status": "\u904B\u884C\u72C0\u614B",
        "server": "\u670D\u52D9\u5668",
        "server_search": "\u641C\u7D22\u670D\u52D9\u5668...",
        "server_empty": "\u672A\u627E\u5230\u670D\u52D9\u5668",
        "reset": "\u91CD\u7F6E",
        "actions": "\u64CD\u4F5C",
        "sort": {
          "tip": "\u62D6\u62FD\u7BC0\u9EDE\u9032\u884C\u6392\u5E8F\uFF0C\u5B8C\u6210\u5F8C\u9EDE\u64CA\u4FDD\u5B58",
          "edit": "\u7DE8\u8F2F\u6392\u5E8F",
          "save": "\u4FDD\u5B58\u6392\u5E8F",
          "success": "\u6392\u5E8F\u4FDD\u5B58\u6210\u529F"
        },
        "batch_delete": {
          "menu": "\u522A\u9664\u7BC0\u9EDE",
          "button": "\u522A\u9664 {{count}} \u9805",
          "title": "\u78BA\u8A8D\u6279\u91CF\u522A\u9664",
          "description": "\u78BA\u5B9A\u8981\u522A\u9664\u9078\u4E2D\u7684 {{count}} \u500B\u7BC0\u9EDE\u55CE\uFF1F\u6B64\u64CD\u4F5C\u4E0D\u53EF\u6062\u5FA9\u3002",
          "confirm": "\u78BA\u8A8D\u522A\u9664"
        },
        "batch_delete_success": "\u6210\u529F\u522A\u9664 {{count}} \u500B\u7BC0\u9EDE",
        "batch_delete_error": "\u6279\u91CF\u522A\u9664\u5931\u6557",
        "batch_show": {
          "menu": "\u986F\u793A\u7BC0\u9EDE"
        },
        "batch_show_success": "\u6210\u529F\u986F\u793A {{count}} \u500B\u7BC0\u9EDE",
        "batch_show_error": "\u6279\u91CF\u986F\u793A\u5931\u6557",
        "batch_hide": {
          "menu": "\u96B1\u85CF\u7BC0\u9EDE"
        },
        "batch_hide_success": "\u6210\u529F\u96B1\u85CF {{count}} \u500B\u7BC0\u9EDE",
        "batch_hide_error": "\u6279\u91CF\u96B1\u85CF\u5931\u6557",
        "batch_enable": {
          "menu": "\u555F\u7528\u7BC0\u9EDE"
        },
        "batch_enable_success": "\u6210\u529F\u555F\u7528 {{count}} \u500B\u7BC0\u9EDE",
        "batch_enable_error": "\u6279\u91CF\u555F\u7528\u5931\u6557",
        "batch_disable": {
          "menu": "\u7981\u7528\u7BC0\u9EDE"
        },
        "batch_disable_success": "\u6210\u529F\u7981\u7528 {{count}} \u500B\u7BC0\u9EDE",
        "batch_disable_error": "\u6279\u91CF\u7981\u7528\u5931\u6557",
        "batch_reset_traffic": {
          "menu": "\u91CD\u7F6E\u6D41\u91CF",
          "button": "\u91CD\u7F6E {{count}} \u9805\u6D41\u91CF",
          "title": "\u78BA\u8A8D\u6279\u91CF\u91CD\u7F6E\u6D41\u91CF",
          "description": "\u78BA\u5B9A\u8981\u91CD\u7F6E\u9078\u4E2D\u7684 {{count}} \u500B\u7BC0\u9EDE\u7684\u6D41\u91CF\u55CE\uFF1F\u6B64\u64CD\u4F5C\u5C07\u6E05\u96F6\u6D41\u91CF\u4E26\u89E3\u9664\u7981\u7528\u72C0\u614B\u3002",
          "confirm": "\u78BA\u8A8D\u91CD\u7F6E"
        },
        "batch_reset_traffic_success": "\u6210\u529F\u91CD\u7F6E {{count}} \u500B\u7BC0\u9EDE\u7684\u6D41\u91CF",
        "batch_reset_traffic_error": "\u6279\u91CF\u91CD\u7F6E\u6D41\u91CF\u5931\u6557",
        "batch_replace": {
          "selected": "\u5DF2\u9078\u64C7 {{count}} \u500B\u7BC0\u9EDE",
          "menu": "\u6279\u91CF\u66FF\u63DB",
          "clear": "\u6E05\u9664\u9078\u64C7",
          "title": "\u6279\u91CF\u66FF\u63DB\u7BC0\u9EDE\u5B57\u6BB5",
          "field": "\u5B57\u6BB5",
          "search": "\u641C\u7D22\u503C",
          "search_placeholder": "\u8F38\u5165\u8981\u641C\u7D22\u7684\u5B57\u7B26\u4E32",
          "replace": "\u66FF\u63DB\u503C",
          "replace_placeholder": "\u8F38\u5165\u8981\u66FF\u63DB\u6210\u7684\u5B57\u7B26\u4E32",
          "confirm": "\u66FF\u63DB",
          "search_required": "\u8ACB\u8F38\u5165\u641C\u7D22\u503C",
          "success": "\u6210\u529F\u66FF\u63DB {{count}} \u500B\u7BC0\u9EDE",
          "fields": {
            "name": "\u7BC0\u9EDE\u540D\u7A31",
            "host": "\u4E3B\u6A5F\u5730\u5740",
            "port": "\u7AEF\u53E3",
            "code": "\u7BC0\u9EDE\u6A19\u8B58",
            "group_ids": "\u6B0A\u9650\u7D44",
            "route_ids": "\u8DEF\u7531\u898F\u5247",
            "tags": "\u6A19\u7C64",
            "protocol_settings": "\u5354\u8B70\u8A2D\u7F6E",
            "custom_outbounds": "\u81EA\u5B9A\u7FA9\u51FA\u7AD9",
            "custom_routes": "\u81EA\u5B9A\u7FA9\u8DEF\u7531",
            "cert_config": "\u8B49\u66F8\u914D\u7F6E",
            "rate_time_ranges": "\u500D\u7387\u6642\u9593\u6BB5"
          }
        },
        "filteringByMachine": "\u6B63\u5728\u7BE9\u9078\u8A72\u6A5F\u5668\u7684\u7BC0\u9EDE\uFF1A",
        "virtualNode": "\u865B\u64EC\u7BC0\u9EDE"
      },
      "form": {
        "add_node": "\u6DFB\u52A0\u7BC0\u9EDE",
        "edit_node": "\u7DE8\u8F2F\u7BC0\u9EDE",
        "new_node": "\u65B0\u5EFA\u7BC0\u9EDE",
        "type": {
          "placeholder": "\u9078\u64C7\u5354\u8B70\u985E\u578B",
          "select_prompt": "\u8ACB\u5148\u9078\u64C7\u5354\u8B70\u985E\u578B",
          "select_error": "\u8ACB\u5148\u9078\u64C7\u5354\u8B70\u985E\u578B",
          "configHint": "\u9078\u64C7\u5354\u8B70\u985E\u578B\u5F8C\u53EF\u914D\u7F6E"
        },
        "name": {
          "label": "\u7BC0\u9EDE\u540D\u7A31",
          "placeholder": "\u8ACB\u8F38\u5165\u7BC0\u9EDE\u540D\u7A31",
          "error": "\u8ACB\u8F38\u5165\u6709\u6548\u7684\u7BC0\u9EDE\u540D\u7A31"
        },
        "rate": {
          "label": "\u57FA\u790E\u500D\u7387",
          "hint": "\u6D41\u91CF\u8A08\u8CBB\u500D\u7387\uFF0C1 \u8868\u793A\u6309\u5BE6\u969B\u6D41\u91CF\u8A08\u8CBB",
          "error": "\u57FA\u790E\u500D\u7387\u4E0D\u80FD\u70BA\u7A7A",
          "error_numeric": "\u57FA\u790E\u500D\u7387\u5FC5\u9808\u662F\u6578\u5B57",
          "error_gte_zero": "\u57FA\u790E\u500D\u7387\u5FC5\u9808\u5927\u65BC\u6216\u7B49\u65BC0",
          "child_node_tooltip": "\u5B50\u7BC0\u9EDE\u7684\u57FA\u790E\u500D\u7387\u7E7C\u627F\u81EA\u7236\u7BC0\u9EDE\uFF0C\u7121\u6CD5\u55AE\u7368\u8A2D\u7F6E",
          "child_node_note": "\u5B50\u7BC0\u9EDE\u500D\u7387\u7E7C\u627F\u81EA\u7236\u7BC0\u9EDE"
        },
        "traffic_limit": {
          "label": "\u6D41\u91CF\u9650\u5236",
          "placeholder": "0 \u8868\u793A\u4E0D\u9650\u5236",
          "hint": "\u8A2D\u7F6E\u7BC0\u9EDE\u6D41\u91CF\u4E0A\u9650\uFF08\u55AE\u4F4D\uFF1AGB\uFF09\uFF0C0 \u8868\u793A\u4E0D\u9650\u5236",
          "error_numeric": "\u6D41\u91CF\u9650\u5236\u5FC5\u9808\u662F\u6578\u5B57",
          "error_gte_zero": "\u6D41\u91CF\u9650\u5236\u5FC5\u9808\u5927\u65BC\u6216\u7B49\u65BC0"
        },
        "protocolSection": "\u5354\u8B70\u914D\u7F6E",
        "traffic_limit_unit": "GB, 0=\u4E0D\u9650\u5236",
        "banned": {
          "label": "\u7981\u7528\u7BC0\u9EDE",
          "description": "\u7981\u7528\u5F8C\u7BC0\u9EDE\u5C07\u4E0D\u53EF\u7528"
        },
        "dynamic_rate": {
          "section_title": "\u52D5\u614B\u500D\u7387\u914D\u7F6E",
          "enable_label": "\u555F\u7528\u52D5\u614B\u500D\u7387",
          "enable_description": "\u6839\u64DA\u6642\u9593\u6BB5\u8A2D\u7F6E\u4E0D\u540C\u7684\u500D\u7387\u4E58\u6578",
          "rules_label": "\u6642\u9593\u6BB5\u898F\u5247",
          "add_rule": "\u6DFB\u52A0\u898F\u5247",
          "rule_title": "\u898F\u5247 {{index}}",
          "start_time": "\u958B\u59CB\u6642\u9593",
          "end_time": "\u7D50\u675F\u6642\u9593",
          "multiplier": "\u500D\u7387\u4E58\u6578",
          "no_rules": "\u66AB\u7121\u898F\u5247\uFF0C\u9EDE\u64CA\u4E0A\u65B9\u6309\u9215\u6DFB\u52A0",
          "start_time_error": "\u958B\u59CB\u6642\u9593\u4E0D\u80FD\u70BA\u7A7A",
          "end_time_error": "\u7D50\u675F\u6642\u9593\u4E0D\u80FD\u70BA\u7A7A",
          "multiplier_error": "\u500D\u7387\u4E58\u6578\u4E0D\u80FD\u70BA\u7A7A",
          "multiplier_error_numeric": "\u500D\u7387\u4E58\u6578\u5FC5\u9808\u662F\u6578\u5B57",
          "multiplier_error_gte_zero": "\u500D\u7387\u4E58\u6578\u5FC5\u9808\u5927\u65BC\u6216\u7B49\u65BC0"
        },
        "required_mark": "\u5FC5\u586B",
        "optional_mark": "\u9078\u586B",
        "required_fields_hint": "\u5E36 * \u7684\u70BA\u5FC5\u586B\u9805\uFF0C\u63D0\u4EA4\u524D\u8ACB\u5B8C\u6574\u586B\u5BEB",
        "code": {
          "label": "\u81EA\u5B9A\u7FA9\u7BC0\u9EDEID",
          "optional": "(\u9078\u586B)",
          "placeholder": "\u8ACB\u8F38\u5165\u81EA\u5B9A\u7FA9\u7BC0\u9EDEID",
          "hint": "\u9078\u586B\uFF1B\u7528\u65BC\u5916\u90E8\u7CFB\u7D71\u5C0D\u7167\uFF0C\u4E0D\u5F71\u97FF\u5354\u8B70\u9023\u63A5"
        },
        "tags": {
          "label": "\u7BC0\u9EDE\u6A19\u7C64",
          "optional": "(\u9078\u586B)",
          "placeholder": "\u8F38\u5165\u5F8C\u6309\u9017\u865F/\u56DE\u8ECA\u6DFB\u52A0",
          "hint": "\u9078\u586B\uFF1B\u5C55\u793A\u5728\u7528\u6236\u7AEF\u7BC0\u9EDE\u540D\u7A31\u65C1"
        },
        "groups": {
          "label": "\u6B0A\u9650\u7D44",
          "add": "\u6DFB\u52A0\u6B0A\u9650\u7D44",
          "placeholder": "\u8ACB\u9078\u64C7\u6B0A\u9650\u7D44",
          "empty": "\u672A\u627E\u5230\u7D50\u679C",
          "hint": "\u9078\u586B\uFF1B\u4E0D\u9078\u5247\u4EFB\u4F55\u6B0A\u9650\u7D44\u7528\u6236\u90FD\u770B\u4E0D\u5230\u6B64\u7BC0\u9EDE\uFF08\u53D6\u6C7A\u65BC\u696D\u52D9\u908F\u8F2F\uFF0C\u5EFA\u8B70\u81F3\u5C11\u9078\u4E00\u500B\uFF09"
        },
        "routes": {
          "label": "\u8DEF\u7531\u898F\u5247",
          "empty": "\u672A\u627E\u5230\u8DEF\u7531\u898F\u5247",
          "hint": "\u9078\u586B\uFF1B\u7D81\u5B9A\u7684\u8DEF\u7531\u898F\u5247\u6703\u4E0B\u767C\u5230\u4F7F\u7528\u8A72\u7BC0\u9EDE\u7684\u5BA2\u6236\u7AEF\u8A02\u95B1\u914D\u7F6E\u4E2D"
        },
        "machine": {
          "label": "\u7D81\u5B9A\u670D\u52D9\u5668",
          "placeholder": "\u9078\u64C7\u670D\u52D9\u5668\uFF08\u53EF\u9078\uFF09",
          "none": "\u7368\u7ACB\u90E8\u7F72",
          "enabled_hint": "\u9078\u64C7\u662F\u5426\u7531\u6B64\u670D\u52D9\u5668\u7BA1\u7406\u8A72\u7BC0\u9EDE",
          "hint": "\u9078\u586B\uFF1B\u7D81\u5B9A\u5F8C\u7531\u8A72\u670D\u52D9\u5668\u4E0A\u7684 Fboard-Node \u8A17\u7BA1\u6B64\u7BC0\u9EDE"
        },
        "host": {
          "label": "\u7BC0\u9EDE\u5730\u5740",
          "placeholder": "\u8ACB\u8F38\u5165\u7BC0\u9EDE\u57DF\u540D\u6216\u8005IP",
          "error": "\u7BC0\u9EDE\u5730\u5740\u4E0D\u80FD\u70BA\u7A7A"
        },
        "port": {
          "label": "\u9023\u63A5\u7AEF\u53E3",
          "placeholder": "443 \u6216 10000-11000",
          "tooltip": "\u7528\u6236\u5BE6\u969B\u9023\u63A5\u4F7F\u7528\u7684\u7AEF\u53E3\u3002\u652F\u6301\u55AE\u7AEF\u53E3\uFF08\u5982 443\uFF09\u6216\u7AEF\u53E3\u7BC4\u570D\uFF08\u5982 10000-11000\uFF09\u3002Hysteria2 \u586B\u5BEB\u7BC4\u570D\u6642\u7BC0\u9EDE\u6703\u5728\u8A72\u7BC4\u570D\u5167\u591A\u7AEF\u53E3\u76E3\u807D\u4EE5\u652F\u6301\u7AEF\u53E3\u8DF3\u8E8D\uFF08\u8DE8\u5EA6\u4E0A\u9650 1024\uFF09\uFF1B\u5176\u4ED6\u5354\u8B70\u7BC4\u570D\u50C5\u7528\u65BC\u8A02\u95B1\u96A8\u6A5F\u9078\u7AEF\u53E3\u3002\u82E5\u4F7F\u7528\u4E2D\u8F49/\u96A7\u9053\uFF0C\u53EF\u8207\u670D\u52D9\u7AEF\u53E3\u4E0D\u540C\u3002",
          "sync": "\u540C\u6B65\u5230\u670D\u52D9\u7AEF\u53E3",
          "error": "\u9023\u63A5\u7AEF\u53E3\u4E0D\u80FD\u70BA\u7A7A"
        },
        "server_port": {
          "label": "\u670D\u52D9\u7AEF\u53E3",
          "placeholder": "\u8ACB\u8F38\u5165\u670D\u52D9\u7AEF\u53E3",
          "error": "\u670D\u52D9\u7AEF\u53E3\u4E0D\u80FD\u70BA\u7A7A",
          "tooltip": "\u670D\u52D9\u5668\u4E0A\u7684\u5BE6\u969B\u76E3\u807D\u7AEF\u53E3\u3002Hysteria2 \u9023\u63A5\u7AEF\u53E3\u70BA\u7BC4\u570D\u6642\uFF0C\u7BC0\u9EDE\u6309\u9023\u63A5\u7AEF\u53E3\u7BC4\u570D\u591A\u7AEF\u53E3\u76E3\u807D\uFF0C\u6B64\u5B57\u6BB5\u4F5C\u70BA\u517C\u5BB9\u56DE\u9000\u7AEF\u53E3\u3002",
          "sync": "\u540C\u6B65\u5230\u670D\u52D9\u7AEF\u53E3"
        },
        "listen_address": {
          "label": "\u76E3\u807D\u5730\u5740",
          "placeholder": "\u7559\u7A7A\u4F7F\u7528\u9ED8\u8A8D (0.0.0.0)\uFF0C\u6216\u8F38\u5165: 127.0.0.1, ::1 \u7B49",
          "description": "\u6307\u5B9A\u670D\u52D9\u5668\u76E3\u807D\u7684 IP \u5730\u5740\u3002\u7559\u7A7A\u5247\u4F7F\u7528\u9ED8\u8A8D\u5730\u5740 (0.0.0.0)\uFF0C\u8868\u793A\u76E3\u807D\u6240\u6709\u7DB2\u7D61\u63A5\u53E3\u3002\u53EF\u8A2D\u7F6E\u70BA 127.0.0.1 (\u50C5\u672C\u5730) \u6216\u7279\u5B9A IP \u5730\u5740",
          "show": "\u76E3\u807D\u5730\u5740",
          "hide": "\u96B1\u85CF\u76E3\u807D\u5730\u5740",
          "optional": "\u53EF\u9078"
        },
        "parent": {
          "label": "\u7236\u7D1A\u7BC0\u9EDE",
          "placeholder": "\u9078\u64C7\u7236\u7BC0\u9EDE",
          "none": "\u7121",
          "hint": "\u9078\u586B\uFF1B\u5B50\u7BC0\u9EDE\u6703\u7E7C\u627F\u7236\u7BC0\u9EDE\u5354\u8B70\u914D\u7F6E\u8207\u5728\u7DDA\u72C0\u614B"
        },
        "virtualNode": {
          "label": "\u865B\u64EC\u7BC0\u9EDE",
          "add": "\u6DFB\u52A0\u865B\u64EC\u7BC0\u9EDE",
          "addDialogTitle": "\u6DFB\u52A0\u865B\u64EC\u7BC0\u9EDE",
          "editDialogTitle": "\u7DE8\u8F2F\u865B\u64EC\u7BC0\u9EDE",
          "edit": "\u7DE8\u8F2F",
          "delete": "\u522A\u9664",
          "save": "\u4FDD\u5B58\u865B\u64EC\u7BC0\u9EDE",
          "saveSuccess": "\u865B\u64EC\u7BC0\u9EDE\u5DF2\u4FDD\u5B58",
          "saveFailed": "\u4FDD\u5B58\u865B\u64EC\u7BC0\u9EDE\u5931\u6557",
          "description": "\u865B\u64EC\u7BC0\u9EDE\u662F\u540C\u4E00\u670D\u52D9\u7684\u984D\u5916\u63A5\u5165\u9EDE\uFF0C\u8207\u5B50\u7236\u7BC0\u9EDE\u662F\u7368\u7ACB\u529F\u80FD",
          "empty": "\u66AB\u7121\u865B\u64EC\u7BC0\u9EDE",
          "tagsPlaceholder": "\u8F38\u5165\u5F8C\u6309\u56DE\u8ECA\u6DFB\u52A0",
          "host": "\u4E3B\u6A5F\u5730\u5740",
          "hostPlaceholder": "\u4E3B\u6A5F\u540D / \u57DF\u540D",
          "port": "\u7AEF\u53E3",
          "groupIds": "\u6B0A\u9650\u7D44",
          "groupIdsHint": "\u53EA\u80FD\u9078\u64C7\u7236\u7BC0\u9EDE\u5DF2\u5305\u542B\u7684\u6B0A\u9650\u7D44\uFF0C\u8D85\u51FA\u7236\u7BC0\u9EDE\u5C07\u7121\u6CD5\u9023\u63A5",
          "groupIdsParentEmpty": "\u8ACB\u5148\u70BA\u7236\u7BC0\u9EDE\u9078\u64C7\u6B0A\u9650\u7D44",
          "tags": "\u6A19\u7C64",
          "show": "\u986F\u793A",
          "visible": "\u986F\u793A",
          "hidden": "\u96B1\u85CF",
          "namePlaceholder": "\u865B\u64EC\u7BC0\u9EDE\u540D\u7A31",
          "deleteConfirmTitle": "\u78BA\u8A8D\u522A\u9664",
          "deleteConfirmDesc": "\u78BA\u5B9A\u8981\u522A\u9664\u865B\u64EC\u7BC0\u9EDE\u300C{{name}}\u300D\u55CE\uFF1F\u6B64\u64CD\u4F5C\u4E0D\u53EF\u6062\u5FA9\u3002",
          "deleteSuccess": "\u865B\u64EC\u7BC0\u9EDE\u5DF2\u522A\u9664",
          "deleteFailed": "\u522A\u9664\u5931\u6557",
          "toggleFailed": "\u66F4\u65B0\u986F\u793A\u72C0\u614B\u5931\u6557",
          "generateKeyPair": "\u751F\u6210\u5BC6\u9470\u5C0D"
        },
        "route": {
          "label": "\u8DEF\u7531\u7D44",
          "placeholder": "\u9078\u64C7\u8DEF\u7531\u7D44",
          "empty": "\u672A\u627E\u5230\u7D50\u679C"
        },
        "submit": "\u63D0\u4EA4",
        "cancel": "\u53D6\u6D88",
        "success": "\u63D0\u4EA4\u6210\u529F"
      },
      "networkTemplate": {
        "title": "\u7DB2\u7D61\u6A21\u677F",
        "empty": "\u7576\u524D\u5354\u8B70\u66AB\u7121\u53EF\u7528\u6A21\u677F",
        "description": "\u9078\u64C7\u4E00\u500B\u9810\u8A2D\u7DB2\u7D61\u6A21\u677F\uFF0C\u4E00\u9375\u586B\u5145\u5354\u8B70\u914D\u7F6E",
        "use": "\u4F7F\u7528",
        "presets": {
          "vless-tcp-vision": {
            "label": "TCP + XTLS Vision",
            "description": "VLESS + TCP + XTLS Vision \u76F4\u9023\uFF0C\u901F\u5EA6\u6975\u5FEB\uFF0C\u63A8\u85A6"
          },
          "vless-ws-tls": {
            "label": "WebSocket + TLS + CDN",
            "description": "VLESS + WebSocket + TLS\uFF0C\u53EF\u5957 CDN\uFF08Cloudflare \u7B49\uFF09"
          },
          "vless-grpc-tls": {
            "label": "gRPC + TLS",
            "description": "VLESS + gRPC + TLS\uFF0C\u9069\u5408\u5927\u898F\u6A21\u8CA0\u8F09"
          },
          "vless-tcp-reality": {
            "label": "TCP + REALITY",
            "description": "VLESS + REALITY \u76F4\u9023\uFF0C\u7121\u9700\u8B49\u66F8\uFF0C\u9632\u4E3B\u52D5\u63A2\u6E2C"
          },
          "trojan-tls": {
            "label": "Trojan + TLS",
            "description": "\u6A19\u6E96 Trojan + TLS\uFF0C\u7AEF\u53E3 443"
          },
          "trojan-ws-tls": {
            "label": "Trojan + WebSocket + TLS",
            "description": "Trojan + WebSocket + TLS\uFF0C\u53EF\u5957 CDN"
          },
          "ss-simple": {
            "label": "\u6A19\u6E96 AEAD",
            "description": "Shadowsocks \u52A0\u5BC6\u96A7\u9053\uFF0C\u7C21\u6F54\u9AD8\u6548"
          },
          "hy-standard": {
            "label": "Hysteria \u6A19\u6E96",
            "description": "Hysteria \u57FA\u65BC QUIC\uFF0C\u6297\u4E1F\u5305\uFF0C\u9069\u5408\u5F31\u7DB2"
          },
          "hy-brutal": {
            "label": "Hysteria2 Brute",
            "description": "Hysteria2 + Brute \u6A21\u5F0F\uFF0C\u6975\u9AD8\u5E36\u5BEC\u5229\u7528"
          },
          "tuic-v5": {
            "label": "TUIC v5 \u6A19\u6E96",
            "description": "TUIC v5 \u57FA\u65BC QUIC\uFF0C\u4F4E\u5EF6\u9072"
          },
          "anytls-default": {
            "label": "AnyTLS \u9ED8\u8A8D",
            "description": "AnyTLS \u81EA\u52D5\u5316 TLS \u507D\u88DD"
          },
          "sudoku-default": {
            "label": "Sudoku \u9ED8\u8A8D",
            "description": "\u4F4E\u71B5\u8868 + ChaCha20 + legacy HTTPMask"
          },
          "shadowquic-default": {
            "label": "ShadowQUIC \u9ED8\u8A8D",
            "description": "JLS \u507D\u88DD + 0-RTT QUIC\uFF0C\u9ED8\u8A8D\u4E0A\u6E38 Cloudflare"
          }
        }
      },
      "dynamic_form": {
        "utls": {
          "fingerprint": {
            "random": "\u96A8\u6A5F\uFF08\u8A02\u95B1\u6642\u62BD\u53D6\uFF09",
            "randomized": "\u96A8\u6A5F\u5316\uFF08\u5BA2\u6236\u7AEF\u5167\u96A8\u6A5F\uFF09"
          }
        },
        "multiplex": {
          "enabled": {
            "label": "\u591A\u8DEF\u8907\u7528 (Multiplex)",
            "description": "\u901A\u904E\u55AE\u689D TCP \u9023\u63A5\u50B3\u8F38\u591A\u500B\u6D41\uFF0C\u964D\u4F4E\u63E1\u624B\u5EF6\u9072"
          },
          "protocol": {
            "label": "\u8907\u7528\u5354\u8B70"
          },
          "max_connections": {
            "label": "\u6700\u5927\u9023\u63A5\u6578"
          },
          "min_streams": {
            "label": "\u6700\u5C0F\u6D41\u6578"
          },
          "padding": {
            "label": "\u555F\u7528\u586B\u5145"
          },
          "brutal": {
            "enabled": {
              "label": "TCP Brutal (\u6FC0\u9032\u64C1\u585E\u63A7\u5236)"
            },
            "up_mbps": {
              "label": "\u4E0A\u884C\u5E36\u5BEC"
            },
            "down_mbps": {
              "label": "\u4E0B\u884C\u5E36\u5BEC"
            },
            "description": "TCP Brutal \u662F\u96D9\u908A\u52A0\u901F\u7B97\u6CD5\uFF0C\u5EFA\u8B70\u5E36\u5BEC\u8A2D\u70BA\u6A5F\u5668\u5BE6\u969B\u5E36\u5BEC\u7684 80%-90%\uFF0C\u958B\u555F\u5F8C BBR \u5C07\u5931\u6548\u3002"
          }
        },
        "ech": {
          "description": "\u70BA\u652F\u6301\u7684 TLS \u5BA2\u6236\u7AEF\u555F\u7528 Encrypted Client Hello\u3002\u7559\u7A7A\u914D\u7F6E\u6642\u6703\u5617\u8A66\u901A\u904E DNS \u67E5\u8A62\u3002",
          "generate": "\u81EA\u52D5\u751F\u6210 ECH \u5BC6\u9470\u5C0D",
          "generateSuccess": "ECH \u5BC6\u9470\u5C0D\u5DF2\u751F\u6210",
          "generateFailed": "\u751F\u6210 ECH \u5BC6\u9470\u5931\u6557",
          "config": {
            "label": "ECH \u914D\u7F6E (PEM)",
            "placeholder": "\u7C98\u8CBC PEM \u683C\u5F0F\u7684 ECH \u914D\u7F6E\uFF0C\u6BCF\u884C\u4E00\u6BB5\u5167\u5BB9",
            "description": "\u7559\u7A7A\u6642\uFF0Csing-box \u6703\u5617\u8A66\u901A\u904E DNS \u52A0\u8F09 ECH \u914D\u7F6E\u3002"
          },
          "config_path": {
            "label": "ECH \u914D\u7F6E\u6587\u4EF6\u8DEF\u5F91",
            "placeholder": "/etc/sing-box/ech.pem",
            "description": "\u6307\u5411 PEM \u683C\u5F0F ECH \u914D\u7F6E\u6587\u4EF6\u7684\u8DEF\u5F91\u3002"
          },
          "query_server_name": {
            "label": "ECH \u67E5\u8A62\u57DF\u540D",
            "placeholder": "\u53EF\u9078\uFF0C\u7528\u65BC\u8986\u84CB HTTPS \u8A18\u9304\u67E5\u8A62\u57DF\u540D",
            "description": "\u8986\u84CB\u7528\u65BC ECH HTTPS \u8A18\u9304\u67E5\u8A62\u7684\u57DF\u540D\uFF0C\u7559\u7A7A\u6642\u9ED8\u8A8D\u4F7F\u7528 server_name\u3002"
          },
          "key": {
            "label": "ECH Key",
            "placeholder": "\u7576\u5F8C\u7AEF\u9700\u8981\u6642\u7C98\u8CBC ECH key \u5167\u5BB9",
            "description": "\u5F8C\u7AEF\u9700\u8981\u6642\u53EF\u586B\u5BEB\u7684 ECH key \u5167\u5BB9\u3002"
          },
          "key_path": {
            "label": "ECH Key \u8DEF\u5F91",
            "placeholder": "/etc/sing-box/ech.key",
            "description": "\u5F8C\u7AEF\u9700\u8981\u6642\u53EF\u586B\u5BEB\u7684 ECH key \u6587\u4EF6\u8DEF\u5F91\u3002"
          }
        },
        "anytls": {
          "tls": {
            "server_name": {
              "label": "\u670D\u52D9\u5668\u540D\u7A31\u6307\u793A(SNI)",
              "placeholder": "\u7576\u7BC0\u9EDE\u5730\u5740\u8207\u8B49\u66F8\u4E0D\u4E00\u81F4\u6642\u7528\u65BC\u8B49\u66F8\u9A57\u8B49"
            },
            "allow_insecure": "\u5141\u8A31\u4E0D\u5B89\u5168\u9023\u63A5"
          },
          "padding_scheme": {
            "label": "\u586B\u5145\u65B9\u6848",
            "placeholder": "\u9078\u64C7\u586B\u5145\u65B9\u6848",
            "edit_btn": "\u7DE8\u8F2F\u586B\u5145\u65B9\u6848",
            "configured": "\u5DF2\u914D\u7F6E {{count}} \u689D\u898F\u5247",
            "not_configured": "\u672A\u914D\u7F6E",
            "description": "\u7528\u65BC\u6DF7\u6DC6\u6D41\u91CF\u7279\u5FB5\u7684\u586B\u5145\u65B9\u6848\uFF0C\u6BCF\u884C\u4E00\u689D\u898F\u5247\uFF0C\u652F\u6301\u901A\u914D\u7B26 *",
            "use_default": "\u4F7F\u7528\u9ED8\u8A8D\u65B9\u6848"
          }
        },
        "shadowsocks": {
          "cipher": {
            "label": "\u52A0\u5BC6\u7B97\u6CD5",
            "placeholder": "\u9078\u64C7\u52A0\u5BC6\u7B97\u6CD5",
            "search_placeholder": "\u641C\u7D22\u6216\u8F38\u5165\u81EA\u5B9A\u7FA9\u52A0\u5BC6\u65B9\u5F0F...",
            "description": "\u9078\u64C7\u9810\u8A2D\u52A0\u5BC6\u65B9\u5F0F\u6216\u8F38\u5165\u81EA\u5B9A\u7FA9\u52A0\u5BC6\u65B9\u5F0F",
            "preset_group": "\u9810\u8A2D\u52A0\u5BC6\u65B9\u5F0F",
            "custom_group": "\u81EA\u5B9A\u7FA9\u52A0\u5BC6\u65B9\u5F0F",
            "current_value": "\u7576\u524D\u503C",
            "use_custom": "\u4F7F\u7528",
            "no_results": "\u672A\u627E\u5230\u5339\u914D\u7684\u52A0\u5BC6\u65B9\u5F0F",
            "custom_hint": "\u4F60\u53EF\u4EE5\u76F4\u63A5\u8F38\u5165\u81EA\u5B9A\u7FA9\u7684\u52A0\u5BC6\u65B9\u5F0F\uFF0C\u5982\uFF1Aaes-256-cfb",
            "custom_label": "\u81EA\u5B9A\u7FA9"
          },
          "plugin": {
            "label": "\u63D2\u4EF6",
            "placeholder": "\u9078\u64C7\u63D2\u4EF6",
            "obfs_hint": "\u63D0\u793A\uFF1A\u914D\u7F6E\u683C\u5F0F\u5982 obfs=http;obfs-host=www.bing.com;path=/",
            "v2ray_hint": "\u63D0\u793A\uFF1AWebSocket\u6A21\u5F0F\u683C\u5F0F\u70BA mode=websocket;host=mydomain.me;path=/;tls=true\uFF0CQUIC\u6A21\u5F0F\u683C\u5F0F\u70BA mode=quic;host=mydomain.me",
            "gost_hint": "\u63D0\u793A\uFF1A\u914D\u7F6E\u683C\u5F0F\u5982 mode=websocket;host=mydomain.me;path=/;tls=true",
            "shadow_tls_hint": "\u63D0\u793A\uFF1A\u914D\u7F6E\u683C\u5F0F\u5982 host=cloud.tencent.com;password=auth_password;version=3",
            "restls_hint": "\u63D0\u793A\uFF1A\u914D\u7F6E\u683C\u5F0F\u5982 host=www.microsoft.com;password=auth_password;version-hint=tls13;restls-script=300?100<1,400~100",
            "kcptun_hint": "\u63D0\u793A\uFF1A\u914D\u7F6E\u683C\u5F0F\u5982 key=psk;crypt=aes-128-gcm;mode=fast;mtu=1350"
          },
          "plugin_opts": {
            "label": "\u63D2\u4EF6\u9078\u9805",
            "description": "\u6309\u7167 key=value;key2=value2 \u683C\u5F0F\u8F38\u5165\u63D2\u4EF6\u9078\u9805",
            "placeholder": "\u4F8B\u5982: mode=tls;host=bing.com"
          },
          "client_fingerprint": "\u5BA2\u6236\u7AEF\u6307\u7D0B",
          "client_fingerprint_placeholder": "\u9078\u64C7\u5BA2\u6236\u7AEF\u6307\u7D0B",
          "client_fingerprint_description": "\u5BA2\u6236\u7AEF\u507D\u88DD\u6307\u7D0B\uFF0C\u7528\u65BC\u964D\u4F4E\u88AB\u8B58\u5225\u98A8\u96AA",
          "obfs": {
            "label": "\u6DF7\u6DC6",
            "placeholder": "\u9078\u64C7\u6DF7\u6DC6\u65B9\u5F0F",
            "none": "\u7121",
            "http": "HTTP"
          },
          "obfs_settings": {
            "path": "\u8DEF\u5F91",
            "host": "Host"
          },
          "cert_config": {
            "tab": "TLS \u8B49\u66F8",
            "cert_mode": {
              "label": "\u8B49\u66F8\u6A21\u5F0F",
              "description": "\u9078\u64C7\u8B49\u66F8\u7533\u8ACB\u65B9\u5F0F\uFF0C\u50C5\u90E8\u5206\u5F8C\u7AEF\u7BC0\u9EDE\u652F\u6301",
              "self_description": "\u81EA\u7C3D\u540D\u6A21\u5F0F\uFF1A\u50C5\u9700\u586B\u5BEB\u57DF\u540D\uFF0C\u8B49\u66F8\u7531\u7BC0\u9EDE\u5F8C\u7AEF\u81EA\u52D5\u751F\u6210\uFF0810\u5E74\u6709\u6548\u671F\uFF09",
              "http_description": "HTTP-01 \u6A21\u5F0F\uFF1A\u9700\u8981 80 \u7AEF\u53E3\u53EF\u6B63\u5E38\u8A2A\u554F\u4EE5\u5B8C\u6210\u8A8D\u8B49",
              "dns_description": "DNS-01 \u6A21\u5F0F\uFF1A\u901A\u904E DNS \u89E3\u6790\u8A18\u9304\u8A8D\u8B49\uFF0C\u652F\u6301\u7533\u8ACB\u6CDB\u57DF\u540D\u8B49\u66F8",
              "content_description": "\u5167\u5BB9\u63A8\u9001\u6A21\u5F0F\uFF1A\u76F4\u63A5\u5C07\u8B49\u66F8\u5167\u5BB9\u4E0B\u767C\u81F3\u7BC0\u9EDE"
            },
            "domain": {
              "label": "\u8B49\u66F8\u57DF\u540D",
              "placeholder": "example.com"
            },
            "email": {
              "label": "\u901A\u77E5\u90F5\u7BB1",
              "placeholder": "admin@example.com"
            },
            "http_port": {
              "label": "\u8A8D\u8B49\u7AEF\u53E3",
              "description": "ACME \u8A8D\u8B49\u7AEF\u53E3 (\u9ED8\u8A8D 80)"
            },
            "dns_provider": {
              "label": "DNS \u63D0\u4F9B\u5546",
              "doc_link": "\u67E5\u770B DNS \u63D0\u4F9B\u5546\u914D\u7F6E\u6307\u5357"
            },
            "dns_env": {
              "label": "\u74B0\u5883\u8B8A\u91CF (API \u5BC6\u9470)",
              "description_short": "\u6BCF\u884C\u4E00\u500B KEY=VALUE \u914D\u7F6E"
            },
            "cert_content": {
              "label": "\u8B49\u66F8\u5167\u5BB9 (PEM)",
              "placeholder": "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----",
              "description": "\u7C98\u8CBC\u5B8C\u6574\u8B49\u66F8 PEM \u6B63\u6587\uFF0C\u4E0D\u662F\u6587\u4EF6\u8DEF\u5F91"
            },
            "key_content": {
              "label": "\u79C1\u9470\u5167\u5BB9 (PEM)",
              "placeholder": "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----",
              "description": "\u7C98\u8CBC\u5B8C\u6574\u79C1\u9470 PEM \u6B63\u6587\uFF0C\u4E0D\u662F\u6587\u4EF6\u8DEF\u5F91"
            },
            "templates": {
              "title": "\u8B49\u66F8\u6A21\u677F",
              "save_current": "\u5132\u5B58\u76EE\u524D\u5167\u5BB9\u70BA\u6A21\u677F",
              "save": "\u5132\u5B58\u6A21\u677F",
              "saved": "\u8B49\u66F8\u6A21\u677F\u5DF2\u5132\u5B58",
              "save_failed": "\u5132\u5B58\u8B49\u66F8\u6A21\u677F\u5931\u6557",
              "deleted": "\u8B49\u66F8\u6A21\u677F\u5DF2\u522A\u9664",
              "delete_failed": "\u522A\u9664\u8B49\u66F8\u6A21\u677F\u5931\u6557",
              "delete": "\u522A\u9664\u6A21\u677F",
              "use": "\u4F7F\u7528",
              "applied": "\u5DF2\u5957\u7528\u8B49\u66F8\u6A21\u677F\uFF1A{{name}}",
              "name_required": "\u8ACB\u8F38\u5165\u6A21\u677F\u540D\u7A31",
              "name_placeholder": "\u6A21\u677F\u540D\u7A31\uFF0C\u4F8B\u5982\uFF1A\u6B63\u5F0F\u8B49\u66F8",
              "description_placeholder": "\u6A21\u677F\u63CF\u8FF0\uFF08\u9078\u586B\uFF09",
              "search_placeholder": "\u641C\u5C0B\u6A21\u677F\u540D\u7A31\u6216\u63CF\u8FF0\u2026",
              "empty_content": "\u8ACB\u5148\u586B\u5BEB\u8B49\u66F8\u5167\u5BB9\u8207\u79C1\u9470\u5167\u5BB9\uFF0C\u518D\u5132\u5B58\u70BA\u6A21\u677F",
              "empty": "\u66AB\u7121\u8B49\u66F8\u6A21\u677F"
            },
            "none_desc": "\u672A\u555F\u7528 TLS \u8B49\u66F8\u914D\u7F6E"
          }
        },
        "vmess": {
          "tls": {
            "label": "TLS",
            "placeholder": "\u8ACB\u9078\u64C7\u5B89\u5168\u6027",
            "disabled": "\u4E0D\u652F\u6301",
            "enabled": "\u652F\u6301"
          },
          "tls_settings": {
            "server_name": {
              "label": "\u670D\u52D9\u5668\u540D\u7A31\u6307\u793A(SNI)",
              "placeholder": "\u4E0D\u4F7F\u7528\u8ACB\u7559\u7A7A"
            },
            "allow_insecure": "\u5141\u8A31\u4E0D\u5B89\u5168?"
          },
          "network": {
            "label": "\u50B3\u8F38\u5354\u8B70",
            "placeholder": "\u9078\u64C7\u50B3\u8F38\u5354\u8B70"
          }
        },
        "trojan": {
          "server_name": {
            "label": "\u670D\u52D9\u5668\u540D\u7A31\u6307\u793A(SNI)",
            "placeholder": "\u7576\u7BC0\u9EDE\u5730\u5740\u65BC\u8B49\u66F8\u4E0D\u4E00\u81F4\u6642\u7528\u65BC\u8B49\u66F8\u9A57\u8B49"
          },
          "allow_insecure": "\u5141\u8A31\u4E0D\u5B89\u5168?",
          "reality_settings": {
            "server_name": {
              "label": "\u507D\u88DD\u7AD9\u9EDE(dest)",
              "placeholder": "\u4F8B\u5982\uFF1Aexample.com"
            },
            "server_port": {
              "label": "\u7AEF\u53E3(port)",
              "placeholder": "\u4F8B\u5982\uFF1A443"
            },
            "allow_insecure": "\u5141\u8A31\u4E0D\u5B89\u5168?",
            "private_key": {
              "label": "\u79C1\u9470(Private key)"
            },
            "public_key": {
              "label": "\u516C\u9470(Public key)"
            },
            "short_id": {
              "label": "Short ID",
              "placeholder": "\u53EF\u7559\u7A7A\uFF0C\u9577\u5EA6\u70BA2\u7684\u500D\u6578\uFF0C\u6700\u957716\u4F4D",
              "description": "\u5BA2\u6236\u7AEF\u53EF\u7528\u7684 shortId \u5217\u8868\uFF0C\u53EF\u7528\u65BC\u5340\u5206\u4E0D\u540C\u7684\u5BA2\u6236\u7AEF\uFF0C\u4F7F\u75280-f\u7684\u5341\u516D\u9032\u5236\u5B57\u7B26",
              "generate": "\u751F\u6210 Short ID",
              "success": "Short ID \u751F\u6210\u6210\u529F"
            },
            "key_pair": {
              "generate": "\u751F\u6210\u5BC6\u9470\u5C0D",
              "success": "\u5BC6\u9470\u5C0D\u751F\u6210\u6210\u529F",
              "error": "\u751F\u6210\u5BC6\u9470\u5C0D\u5931\u6557"
            }
          },
          "network": {
            "label": "\u50B3\u8F38\u5354\u8B70",
            "placeholder": "\u9078\u64C7\u50B3\u8F38\u5354\u8B70"
          }
        },
        "hysteria": {
          "version": {
            "label": "\u5354\u8B70\u7248\u672C",
            "placeholder": "\u5354\u8B70\u7248\u672C"
          },
          "alpn": {
            "label": "ALPN",
            "placeholder": "ALPN"
          },
          "obfs": {
            "label": "\u6DF7\u6DC6",
            "type": {
              "label": "\u6DF7\u6DC6\u5BE6\u73FE",
              "placeholder": "\u9078\u64C7\u6DF7\u6DC6\u5BE6\u73FE",
              "salamander": "Salamander"
            },
            "password": {
              "label": "\u6DF7\u6DC6\u5BC6\u78BC",
              "placeholder": "\u8ACB\u8F38\u5165\u6DF7\u6DC6\u5BC6\u78BC",
              "generate_success": "\u6DF7\u6DC6\u5BC6\u78BC\u751F\u6210\u6210\u529F"
            }
          },
          "tls": {
            "server_name": {
              "label": "\u670D\u52D9\u5668\u540D\u7A31\u6307\u793A(SNI)",
              "placeholder": "\u7576\u7BC0\u9EDE\u5730\u5740\u65BC\u8B49\u66F8\u4E0D\u4E00\u81F4\u6642\u7528\u65BC\u8B49\u66F8\u9A57\u8B49"
            },
            "allow_insecure": "\u5141\u8A31\u4E0D\u5B89\u5168?"
          },
          "bandwidth": {
            "up": {
              "label": "\u4E0A\u884C\u5BEC\u5E36",
              "placeholder": "\u8ACB\u8F38\u5165\u4E0A\u884C\u5BEC\u5E36",
              "suffix": "Mbps",
              "bbr_tip": "\uFF0C\u7559\u7A7A\u5247\u4F7F\u7528BBR"
            },
            "down": {
              "label": "\u4E0B\u884C\u5BEC\u5E36",
              "placeholder": "\u8ACB\u8F38\u5165\u4E0B\u884C\u5BEC\u5E36",
              "suffix": "Mbps",
              "bbr_tip": "\uFF0C\u7559\u7A7A\u5247\u4F7F\u7528BBR"
            }
          }
        },
        "vless": {
          "tls": {
            "label": "\u5B89\u5168\u6027",
            "placeholder": "\u8ACB\u9078\u64C7\u5B89\u5168\u6027",
            "none": "\u7121",
            "tls": "TLS",
            "reality": "Reality"
          },
          "tls_settings": {
            "server_name": {
              "label": "\u670D\u52D9\u5668\u540D\u7A31\u6307\u793A(SNI)",
              "placeholder": "\u4E0D\u4F7F\u7528\u8ACB\u7559\u7A7A"
            },
            "allow_insecure": "\u5141\u8A31\u4E0D\u5B89\u5168?"
          },
          "reality_settings": {
            "server_name": {
              "label": "\u507D\u88DD\u7AD9\u9EDE(dest)",
              "placeholder": "\u4F8B\u5982\uFF1Aexample.com"
            },
            "server_port": {
              "label": "\u7AEF\u53E3(port)",
              "placeholder": "\u4F8B\u5982\uFF1A443"
            },
            "allow_insecure": "\u5141\u8A31\u4E0D\u5B89\u5168?",
            "private_key": {
              "label": "\u79C1\u9470(Private key)"
            },
            "public_key": {
              "label": "\u516C\u9470(Public key)"
            },
            "short_id": {
              "label": "Short ID",
              "placeholder": "\u53EF\u7559\u7A7A\uFF0C\u9577\u5EA6\u70BA2\u7684\u500D\u6578\uFF0C\u6700\u957716\u4F4D",
              "description": "\u5BA2\u6236\u7AEF\u53EF\u7528\u7684 shortId \u5217\u8868\uFF0C\u53EF\u7528\u65BC\u5340\u5206\u4E0D\u540C\u7684\u5BA2\u6236\u7AEF\uFF0C\u4F7F\u75280-f\u7684\u5341\u516D\u9032\u5236\u5B57\u7B26",
              "generate": "\u751F\u6210 Short ID",
              "success": "Short ID \u751F\u6210\u6210\u529F"
            },
            "key_pair": {
              "generate": "\u751F\u6210\u5BC6\u9470\u5C0D",
              "success": "\u5BC6\u9470\u5C0D\u751F\u6210\u6210\u529F",
              "error": "\u751F\u6210\u5BC6\u9470\u5C0D\u5931\u6557"
            }
          },
          "network": {
            "label": "\u50B3\u8F38\u5354\u8B70",
            "placeholder": "\u9078\u64C7\u50B3\u8F38\u5354\u8B70"
          },
          "flow": {
            "label": "\u6D41\u63A7",
            "placeholder": "\u9078\u64C7\u6D41\u63A7"
          },
          "encryption": {
            "label": "VLESS Encryption",
            "description": "\u555F\u7528 VLESS \u52A0\u5BC6",
            "server_label": "decryption",
            "server_placeholder": "./xray vlessenc \u751F\u6210",
            "server_description": "\u670D\u52D9\u7AEF decryption \u53C3\u6578\uFF0C\u53EF\u7531 ./xray vlessenc \u751F\u6210",
            "client_label": "encryption",
            "client_placeholder": "./xray vlessenc \u751F\u6210",
            "client_description": "\u5BA2\u6236\u7AEF encryption \u53C3\u6578\uFF0C\u9700\u8207\u670D\u52D9\u7AEF\u914D\u5C0D",
            "generate_hint": "./xray vlessenc \u751F\u6210"
          }
        },
        "tuic": {
          "version": {
            "label": "\u5354\u8B70\u7248\u672C",
            "placeholder": "\u9078\u64C7TUIC\u7248\u672C"
          },
          "password": {
            "label": "\u5BC6\u78BC",
            "placeholder": "\u8ACB\u8F38\u5165\u5BC6\u78BC",
            "generate_success": "\u5BC6\u78BC\u751F\u6210\u6210\u529F"
          },
          "congestion_control": {
            "label": "\u64C1\u585E\u63A7\u5236",
            "placeholder": "\u9078\u64C7\u64C1\u585E\u63A7\u5236\u7B97\u6CD5"
          },
          "udp_relay_mode": {
            "label": "UDP\u4E2D\u7E7C\u6A21\u5F0F",
            "placeholder": "\u9078\u64C7UDP\u4E2D\u7E7C\u6A21\u5F0F"
          },
          "tls": {
            "server_name": {
              "label": "\u670D\u52D9\u5668\u540D\u7A31\u6307\u793A(SNI)",
              "placeholder": "\u7576\u7BC0\u9EDE\u5730\u5740\u8207\u8B49\u66F8\u4E0D\u4E00\u81F4\u6642\u7528\u65BC\u8B49\u66F8\u9A57\u8B49"
            },
            "allow_insecure": "\u5141\u8A31\u4E0D\u5B89\u5168?",
            "alpn": {
              "label": "ALPN",
              "placeholder": "\u9078\u64C7ALPN\u5354\u8B70",
              "empty": "\u672A\u627E\u5230\u53EF\u7528\u7684ALPN\u5354\u8B70"
            }
          }
        },
        "socks": {
          "version": {
            "label": "\u5354\u8B70\u7248\u672C",
            "placeholder": "\u9078\u64C7SOCKS\u7248\u672C"
          },
          "tls": {
            "label": "TLS",
            "placeholder": "\u8ACB\u9078\u64C7\u5B89\u5168\u6027",
            "disabled": "\u4E0D\u652F\u6301",
            "enabled": "\u652F\u6301"
          },
          "tls_settings": {
            "server_name": {
              "label": "\u670D\u52D9\u5668\u540D\u7A31\u6307\u793A(SNI)",
              "placeholder": "\u4E0D\u4F7F\u7528\u8ACB\u7559\u7A7A"
            },
            "allow_insecure": "\u5141\u8A31\u4E0D\u5B89\u5168?"
          },
          "network": {
            "label": "\u50B3\u8F38\u5354\u8B70",
            "placeholder": "\u9078\u64C7\u50B3\u8F38\u5354\u8B70"
          }
        },
        "sudoku": {
          "name": "Sudoku",
          "keyPairTitle": "Sudoku \u5BC6\u9470\u5C0D",
          "keyPairDescription": "\u670D\u52D9\u7AEF\u4F7F\u7528 Master Public Key\uFF1BMaster Private Key \u50C5\u4FDD\u5B58\u5728\u9762\u677F\u7528\u65BC\u6D3E\u751F\u7528\u6236\u5BC6\u9470\uFF0C\u4E0D\u6703\u4E0B\u767C\u5230\u7BC0\u9EDE",
          "generate": "\u4E00\u9375\u751F\u6210",
          "generateSuccess": "Sudoku \u5BC6\u9470\u5C0D\u5DF2\u751F\u6210",
          "generateFailed": "\u751F\u6210 Sudoku \u5BC6\u9470\u5931\u6557",
          "publicPlaceholder": "\u9EDE\u64CA\u53F3\u4E0A\u89D2\u300C\u4E00\u9375\u751F\u6210\u300D\u81EA\u52D5\u586B\u5BEB",
          "privatePlaceholder": "\u50C5\u9762\u677F\u4FDD\u5B58\uFF0C\u52FF\u6D29\u9732",
          "master_public_key": "Master \u516C\u9470",
          "master_private_key": "Master \u79C1\u9470",
          "aead_method": "AEAD \u7B97\u6CD5",
          "padding_min": "\u6700\u5C0F\u586B\u5145\u7387",
          "padding_max": "\u6700\u5927\u586B\u5145\u7387",
          "table_type": "\u8868\u985E\u578B",
          "enable_pure_downlink": "\u7D14 Sudoku \u4E0B\u884C",
          "custom_table": "\u81EA\u5B9A\u7FA9\u8868",
          "custom_tables": "\u81EA\u5B9A\u7FA9\u8868\u5217\u8868",
          "handshake_timeout": "\u63E1\u624B\u8D85\u6642",
          "fallback": "\u56DE\u843D\u5730\u5740",
          "multiplex": "\u591A\u8DEF\u8907\u7528",
          "httpmask": "HTTPMask"
        },
        "naive": {
          "tls_settings": {
            "server_name": {
              "label": "\u670D\u52D9\u5668\u540D\u7A31\u6307\u793A(SNI)",
              "placeholder": "\u4E0D\u4F7F\u7528\u8ACB\u7559\u7A7A"
            },
            "allow_insecure": "\u5141\u8A31\u4E0D\u5B89\u5168?"
          },
          "tls": {
            "label": "TLS",
            "placeholder": "\u8ACB\u9078\u64C7\u5B89\u5168\u6027",
            "disabled": "\u4E0D\u652F\u6301",
            "enabled": "\u652F\u6301",
            "server_name": {
              "label": "\u670D\u52D9\u5668\u540D\u7A31\u6307\u793A(SNI)",
              "placeholder": "\u7576\u7BC0\u9EDE\u5730\u5740\u8207\u8B49\u66F8\u4E0D\u4E00\u81F4\u6642\u7528\u65BC\u8B49\u66F8\u9A57\u8B49"
            },
            "allow_insecure": "\u5141\u8A31\u4E0D\u5B89\u5168\u9023\u63A5"
          }
        },
        "http": {
          "tls": {
            "label": "TLS",
            "placeholder": "\u8ACB\u9078\u64C7\u5B89\u5168\u6027",
            "disabled": "\u4E0D\u652F\u6301",
            "enabled": "\u652F\u6301",
            "server_name": {
              "label": "\u670D\u52D9\u5668\u540D\u7A31\u6307\u793A(SNI)",
              "placeholder": "\u7576\u7BC0\u9EDE\u5730\u5740\u8207\u8B49\u66F8\u4E0D\u4E00\u81F4\u6642\u7528\u65BC\u8B49\u66F8\u9A57\u8B49"
            },
            "allow_insecure": "\u5141\u8A31\u4E0D\u5B89\u5168\u9023\u63A5"
          },
          "tls_settings": {
            "server_name": {
              "label": "\u670D\u52D9\u5668\u540D\u7A31\u6307\u793A(SNI)",
              "placeholder": "\u7576\u7BC0\u9EDE\u5730\u5740\u8207\u8B49\u66F8\u4E0D\u4E00\u81F4\u6642\u7528\u65BC\u8B49\u66F8\u9A57\u8B49"
            },
            "allow_insecure": "\u5141\u8A31\u4E0D\u5B89\u5168\u9023\u63A5"
          }
        },
        "mieru": {
          "transport": {
            "label": "\u50B3\u8F38\u5354\u8B70",
            "placeholder": "\u9078\u64C7\u50B3\u8F38\u5354\u8B70"
          },
          "traffic_pattern": {
            "label": "\u6D41\u91CF\u7279\u5FB5\u507D\u88DD (Base64)",
            "placeholder": "\u7559\u7A7A\u4F7F\u7528\u9ED8\u8A8D\uFF1B\u6216\u9EDE\u64CA\u53F3\u5074\u6309\u9215\u751F\u6210",
            "description": "\u53EF\u9078\u3002\u5B98\u65B9 mieru Traffic Pattern \u7684 Base64 \u4E32\uFF0C\u7528\u65BC TCP \u5206\u7247 / Nonce \u524D\u7DB4\u7B49\u6297 DPI \u6D41\u91CF\u6574\u5F62\uFF1B\u53EF\u7528 mita/mieru export traffic-pattern \u5C0E\u51FA\u3002",
            "generate": "\u751F\u6210\u6D41\u91CF\u7279\u5FB5\u507D\u88DD",
            "success": "\u6D41\u91CF\u7279\u5FB5\u507D\u88DD\u5DF2\u751F\u6210"
          }
        },
        "trusttunnel": {
          "client_random_auth_key": {
            "generate": "\u751F\u6210\u96A8\u6A5F Auth Key\uFF0816 \u5B57\u7BC0 hex\uFF09"
          }
        },
        "cert_config": {
          "tab": "TLS \u8B49\u66F8",
          "none_desc": "\u672A\u555F\u7528 TLS \u8B49\u66F8\u914D\u7F6E",
          "cert_mode": {
            "label": "\u8B49\u66F8\u6A21\u5F0F",
            "description": "\u9078\u64C7\u8B49\u66F8\u7533\u8ACB\u65B9\u5F0F\uFF0C\u50C5\u90E8\u5206\u5F8C\u7AEF\u7BC0\u9EDE\u652F\u6301",
            "none_desc": "\u672A\u555F\u7528 TLS \u8B49\u66F8\u914D\u7F6E"
          },
          "domain": {
            "label": "\u8B49\u66F8\u57DF\u540D"
          },
          "email": {
            "label": "\u901A\u77E5\u90F5\u7BB1"
          },
          "http_port": {
            "label": "\u6311\u6230\u7AEF\u53E3",
            "description": "ACME \u6311\u6230\u7AEF\u53E3 (\u9ED8\u8A8D 80)"
          },
          "dns_provider": {
            "label": "DNS \u63D0\u4F9B\u5546",
            "doc_link": "\u67E5\u770B DNS \u63D0\u4F9B\u5546\u914D\u7F6E\u6307\u5357"
          },
          "dns_env": {
            "label": "\u74B0\u5883\u8B8A\u91CF (API \u5BC6\u9470)",
            "description_short": "\u6BCF\u884C\u4E00\u500B KEY=VALUE \u914D\u7F6E"
          },
          "cert_content": {
            "label": "\u8B49\u66F8\u5167\u5BB9 (PEM)",
            "placeholder": "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----",
            "description": "\u7C98\u8CBC\u5B8C\u6574\u8B49\u66F8 PEM \u6B63\u6587\uFF0C\u4E0D\u662F\u6587\u4EF6\u8DEF\u5F91"
          },
          "key_content": {
            "label": "\u79C1\u9470\u5167\u5BB9 (PEM)",
            "placeholder": "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----",
            "description": "\u7C98\u8CBC\u5B8C\u6574\u79C1\u9470 PEM \u6B63\u6587\uFF0C\u4E0D\u662F\u6587\u4EF6\u8DEF\u5F91"
          },
          "templates": {
            "title": "\u8B49\u66F8\u6A21\u677F",
            "save_current": "\u5132\u5B58\u76EE\u524D\u5167\u5BB9\u70BA\u6A21\u677F",
            "save": "\u5132\u5B58\u6A21\u677F",
            "saved": "\u8B49\u66F8\u6A21\u677F\u5DF2\u5132\u5B58",
            "save_failed": "\u5132\u5B58\u8B49\u66F8\u6A21\u677F\u5931\u6557",
            "deleted": "\u8B49\u66F8\u6A21\u677F\u5DF2\u522A\u9664",
            "delete_failed": "\u522A\u9664\u8B49\u66F8\u6A21\u677F\u5931\u6557",
            "delete": "\u522A\u9664\u6A21\u677F",
            "use": "\u4F7F\u7528",
            "applied": "\u5DF2\u5957\u7528\u8B49\u66F8\u6A21\u677F\uFF1A{{name}}",
            "name_required": "\u8ACB\u8F38\u5165\u6A21\u677F\u540D\u7A31",
            "name_placeholder": "\u6A21\u677F\u540D\u7A31\uFF0C\u4F8B\u5982\uFF1A\u6B63\u5F0F\u8B49\u66F8",
            "description_placeholder": "\u6A21\u677F\u63CF\u8FF0\uFF08\u9078\u586B\uFF09",
            "search_placeholder": "\u641C\u5C0B\u6A21\u677F\u540D\u7A31\u6216\u63CF\u8FF0\u2026",
            "empty_content": "\u8ACB\u5148\u586B\u5BEB\u8B49\u66F8\u5167\u5BB9\u8207\u79C1\u9470\u5167\u5BB9\uFF0C\u518D\u5132\u5B58\u70BA\u6A21\u677F",
            "empty": "\u66AB\u7121\u8B49\u66F8\u6A21\u677F"
          }
        },
        "routing": {
          "outbounds_tab": "\u81EA\u5B9A\u7FA9 Outbounds",
          "routes_tab": "\u81EA\u5B9A\u7FA9 Routes",
          "outbounds": "\u81EA\u5B9A\u7FA9 Outbounds (JSON)",
          "routes": "\u81EA\u5B9A\u7FA9 Routes (JSON)",
          "error": {
            "must_be_array": "\u5FC5\u9808\u662F\u4E00\u500B JSON \u6578\u7D44 []",
            "invalid_json": "\u7121\u6548\u7684 JSON \u683C\u5F0F"
          }
        },
        "advanced": {
          "trigger_label": "\u9AD8\u7D1A\u8A2D\u7F6E",
          "dialog_title": "\u9AD8\u7D1A\u5354\u8B70\u914D\u7F6E",
          "tls_tab": "TLS",
          "route_tab": "\u8DEF\u7531",
          "multiplex_tab": "\u591A\u8DEF\u8907\u7528"
        }
      },
      "network_settings": {
        "edit_protocol": "\u7DE8\u8F2F\u5354\u8B70",
        "edit_protocol_config": "\u7DE8\u8F2F\u7DB2\u7D61\u8A2D\u7F6E",
        "edit_padding_scheme": "\u7DE8\u8F2F\u586B\u5145\u65B9\u6848",
        "use_template": "\u4F7F\u7528{{template}}\u6A21\u677F",
        "json_label": "\u7DB2\u7D61\u8A2D\u7F6E (JSON)",
        "json_config_placeholder": "\u8ACB\u8F38\u5165 JSON \u683C\u5F0F\u7684\u7DB2\u7D61\u8A2D\u7F6E",
        "json_config_placeholder_with_template": "\u8ACB\u8F38\u5165 JSON \u914D\u7F6E\uFF0C\u4F8B\u5982 path / host / serviceName \u7B49",
        "templates": "\u6A21\u677F\u5217\u8868",
        "templates_empty": "\u66AB\u7121\u53EF\u7528\u6A21\u677F\uFF0C\u53EF\u5C07\u7576\u524D\u914D\u7F6E\u4FDD\u5B58\u70BA\u6A21\u677F",
        "templates_custom": "\u6211\u7684\u6A21\u677F",
        "templates_builtin": "\u5167\u7F6E\u6A21\u677F",
        "builtin": "\u5167\u7F6E",
        "save_as_template": "\u4FDD\u5B58\u7576\u524D\u70BA\u6A21\u677F",
        "save_template": "\u4FDD\u5B58\u6A21\u677F",
        "delete_template": "\u522A\u9664\u6A21\u677F",
        "use_template_btn": "\u4F7F\u7528",
        "template_name_placeholder": "\u6A21\u677F\u540D\u7A31\uFF0C\u4F8B\u5982\uFF1AWS \u81EA\u5B9A\u7FA9\u8DEF\u5F91",
        "template_name_required": "\u8ACB\u8F38\u5165\u6A21\u677F\u540D\u7A31",
        "template_empty": "\u7576\u524D\u914D\u7F6E\u70BA\u7A7A\uFF0C\u7121\u6CD5\u4FDD\u5B58\u70BA\u6A21\u677F",
        "template_from_current": "\u4F86\u81EA\u7576\u524D {{network}} \u914D\u7F6E",
        "template_custom_desc": "\u81EA\u5B9A\u7FA9\u7DB2\u7D61\u8A2D\u7F6E\u6A21\u677F",
        "template_saved": "\u6A21\u677F\u5DF2\u4FDD\u5B58",
        "template_save_failed": "\u4FDD\u5B58\u6A21\u677F\u5931\u6557",
        "template_deleted": "\u6A21\u677F\u5DF2\u522A\u9664",
        "template_delete_failed": "\u522A\u9664\u6A21\u677F\u5931\u6557",
        "template_applied": "\u5DF2\u61C9\u7528\u6A21\u677F\uFF1A{{name}}",
        "validation": {
          "must_be_array": "\u914D\u7F6E\u5FC5\u9808\u662F\u4E00\u500BJSON\u6578\u7D44",
          "must_be_object": "\u914D\u7F6E\u5FC5\u9808\u662F\u4E00\u500BJSON\u5C0D\u8C61",
          "invalid_json": "\u7121\u6548\u7684JSON\u683C\u5F0F"
        },
        "errors": {
          "save_failed": "\u4FDD\u5B58\u6642\u767C\u751F\u932F\u8AA4"
        },
        "builtin_templates": {
          "ws-basic": {
            "name": "WebSocket \u57FA\u790E",
            "description": "path + Host"
          },
          "ws-cdn": {
            "name": "WebSocket + CDN",
            "description": "\u9069\u5408 Cloudflare \u7B49 CDN"
          },
          "grpc-basic": {
            "name": "gRPC \u57FA\u790E",
            "description": "serviceName"
          },
          "grpc-multi": {
            "name": "gRPC multiMode",
            "description": "\u958B\u555F multiMode"
          },
          "http-basic": {
            "name": "HTTP/2 \u57FA\u790E",
            "description": "path + host"
          },
          "tcp-none": {
            "name": "TCP \u7121\u507D\u88DD",
            "description": "\u7D14 TCP"
          },
          "tcp-http": {
            "name": "TCP HTTP \u507D\u88DD",
            "description": "header type = http"
          },
          "xhttp-basic": {
            "name": "XHTTP \u57FA\u790E",
            "description": "path \u6A21\u5F0F"
          },
          "quic-basic": {
            "name": "QUIC \u57FA\u790E",
            "description": "security + key"
          },
          "kcp-basic": {
            "name": "KCP \u57FA\u790E",
            "description": "mtu / tti \u9ED8\u8A8D"
          }
        }
      },
      "common": {
        "cancel": "\u53D6\u6D88",
        "confirm": "\u78BA\u5B9A"
      },
      "description": "\u914D\u7F6E\u7BC0\u9EDE\u901A\u4FE1\u8207\u540C\u6B65\u8A2D\u7F6E\uFF0C\u5305\u62EC\u901A\u4FE1\u5BC6\u9470\u3001\u8F2A\u8A62\u9593\u9694\u3001\u8CA0\u8F09\u5747\u8861\u7B49\u9AD8\u7D1A\u9078\u9805\u3002",
      "device_limit_mode": {
        "description": "\u5BEC\u9B06\u6A21\u5F0F\u4E0B\uFF0C\u540C\u4E00 IP \u5730\u5740\u7684\u591A\u500B\u7BC0\u9EDE\u8A08\u70BA 1 \u81FA\u8A2D\u5099\u3002",
        "placeholder": "\u8ACB\u9078\u64C7\u8A2D\u5099\u9650\u5236\u6A21\u5F0F",
        "relaxed": "\u5BEC\u9B06\u6A21\u5F0F",
        "strict": "\u56B4\u683C\u6A21\u5F0F",
        "title": "\u8A2D\u5099\u9650\u5236\u6A21\u5F0F"
      },
      "saving": "\u4FDD\u5B58\u4E2D...",
      "server_pull_interval": {
        "description": "\u7BC0\u9EDE\u5F9E\u9762\u677F\u62C9\u53D6\u6578\u64DA\u7684\u983B\u7387\u3002",
        "placeholder": "\u8ACB\u8F38\u5165\u62C9\u53D6\u9593\u9694",
        "title": "\u7BC0\u9EDE\u62C9\u53D6\u8F2A\u8A62\u9593\u9694"
      },
      "server_push_interval": {
        "description": "\u7BC0\u9EDE\u5411\u9762\u677F\u63A8\u9001\u6578\u64DA\u7684\u983B\u7387\u3002",
        "placeholder": "\u8ACB\u8F38\u5165\u63A8\u9001\u9593\u9694",
        "title": "\u7BC0\u9EDE\u63A8\u9001\u8F2A\u8A62\u9593\u9694"
      },
      "server_token": {
        "description": "\u9762\u677F\u8207\u7BC0\u9EDE\u901A\u4FE1\u7684\u5BC6\u9470\uFF0C\u7528\u65BC\u9632\u6B62\u672A\u6388\u6B0A\u8A2A\u554F\u3002",
        "placeholder": "\u8ACB\u8F38\u5165\u901A\u4FE1\u5BC6\u9470",
        "title": "\u901A\u4FE1\u5BC6\u9470"
      },
      "title": "\u7BC0\u9EDE\u914D\u7F6E",
      "messages": {
        "saveFailed": "\u4FDD\u5B58\u5931\u6557"
      }
    },
    "coupon": {
      "title": "\u512A\u60E0\u5238\u7BA1\u7406",
      "description": "\u5728\u9019\u88E1\u53EF\u4EE5\u67E5\u770B\u512A\u60E0\u5238\uFF0C\u5305\u62EC\u589E\u52A0\u3001\u67E5\u770B\u3001\u522A\u9664\u7B49\u64CD\u4F5C\u3002",
      "table": {
        "columns": {
          "id": "ID",
          "select": "\u9078\u64C7",
          "show": "\u555F\u7528",
          "name": "\u5377\u540D\u7A31",
          "type": "\u985E\u578B",
          "code": "\u5377\u78BC",
          "limitUse": "\u5269\u9918\u6B21\u6578",
          "limitUseWithUser": "\u53EF\u7528\u6B21\u6578/\u7528\u6236",
          "validity": "\u6709\u6548\u671F",
          "actions": "\u64CD\u4F5C"
        },
        "validity": {
          "expired": "\u5DF2\u904E\u671F{{days}}\u5929",
          "notStarted": "{{days}}\u5929\u540E\u958B\u59CB",
          "remaining": "\u5269\u9918{{days}}\u5929",
          "startTime": "\u958B\u59CB\u6642\u9593",
          "endTime": "\u7D50\u675F\u6642\u9593",
          "unlimited": "\u7121\u9650\u6B21",
          "noLimit": "\u7121\u9650\u5236"
        },
        "actions": {
          "edit": "\u7DE8\u8F2F",
          "delete": "\u522A\u9664",
          "deleteConfirm": {
            "title": "\u78BA\u8A8D\u522A\u9664",
            "description": "\u6B64\u64CD\u4F5C\u5C07\u6C38\u4E45\u522A\u9664\u8A72\u512A\u60E0\u5238\uFF0C\u522A\u9664\u5F8C\u7121\u6CD5\u6062\u5FA9\u3002\u78BA\u5B9A\u8981\u7E7C\u7E8C\u55CE\uFF1F",
            "confirmText": "\u522A\u9664"
          },
          "batchDelete": "\u6279\u91CF\u522A\u9664 ({{count}})",
          "batchDeleteConfirm": {
            "title": "\u6279\u91CF\u522A\u9664",
            "description": "\u78BA\u5B9A\u8981\u522A\u9664\u9078\u4E2D\u7684 {{count}} \u5F35\u512A\u60E0\u5238\u55CE\uFF1F\u522A\u9664\u5F8C\u7121\u6CD5\u6062\u5FA9\u3002"
          },
          "dropExpired": "\u522A\u9664\u5DF2\u904E\u671F",
          "dropExpiredConfirm": {
            "title": "\u522A\u9664\u5DF2\u904E\u671F\u512A\u60E0\u5238",
            "description": "\u78BA\u5B9A\u8981\u522A\u9664\u6240\u6709\u5DF2\u904E\u671F\u7684\u512A\u60E0\u5238\u55CE\uFF1F\u522A\u9664\u5F8C\u7121\u6CD5\u6062\u5FA9\u3002"
          },
          "dropExpiredResult": {
            "success": "\u5DF2\u522A\u9664 {{count}} \u5F35\u904E\u671F\u512A\u60E0\u5238",
            "empty": "\u6C92\u6709\u627E\u5230\u5DF2\u904E\u671F\u7684\u512A\u60E0\u5238"
          },
          "dropDepleted": "\u522A\u9664\u5DF2\u7528\u76E1",
          "dropDepletedConfirm": {
            "title": "\u522A\u9664\u5DF2\u7528\u76E1\u512A\u60E0\u5238",
            "description": "\u78BA\u5B9A\u8981\u522A\u9664\u6240\u6709\u5269\u9918\u6B21\u6578\u70BA 0 \u7684\u512A\u60E0\u5238\u55CE\uFF1F\u522A\u9664\u5F8C\u7121\u6CD5\u6062\u5FA9\u3002"
          },
          "dropDepletedResult": {
            "success": "\u5DF2\u522A\u9664 {{count}} \u5F35\u5DF2\u7528\u76E1\u512A\u60E0\u5238",
            "empty": "\u6C92\u6709\u627E\u5230\u5269\u9918\u6B21\u6578\u70BA 0 \u7684\u512A\u60E0\u5238"
          }
        },
        "toolbar": {
          "search": "\u641C\u7D22\u512A\u60E0\u5238...",
          "type": "\u985E\u578B",
          "reset": "\u91CD\u7F6E",
          "types": {
            "1": "\u6309\u91D1\u984D\u512A\u60E0",
            "2": "\u6309\u6BD4\u4F8B\u512A\u60E0"
          }
        }
      },
      "form": {
        "add": "\u6DFB\u52A0\u512A\u60E0\u5238",
        "edit": "\u7DE8\u8F2F\u512A\u60E0\u5238",
        "name": {
          "label": "\u512A\u60E0\u5238\u540D\u7A31",
          "placeholder": "\u8ACB\u8F38\u5165\u512A\u60E0\u5238\u540D\u7A31",
          "required": "\u8ACB\u8F38\u5165\u512A\u60E0\u5238\u540D\u7A31"
        },
        "type": {
          "label": "\u512A\u60E0\u5238\u985E\u578B\u548C\u503C",
          "placeholder": "\u512A\u60E0\u5238\u985E\u578B"
        },
        "value": {
          "placeholder": "\u8ACB\u8F38\u5165\u91D1\u984D\uFF08\u5143\uFF09"
        },
        "validity": {
          "label": "\u512A\u60E0\u5238\u6709\u6548\u671F",
          "to": "\u81F3",
          "endTimeError": "\u7D50\u675F\u6642\u9593\u5FC5\u9808\u665A\u65BC\u958B\u59CB\u6642\u9593"
        },
        "limitUse": {
          "label": "\u6700\u5927\u4F7F\u7528\u6B21\u6578",
          "placeholder": "\u9650\u5236\u6700\u5927\u4F7F\u7528\u6B21\u6578\uFF0C\u7559\u7A7A\u5247\u4E0D\u9650\u5236",
          "description": "\u8A2D\u7F6E\u512A\u60E0\u5238\u7684\u7E3D\u4F7F\u7528\u6B21\u6578\u9650\u5236\uFF0C\u7559\u7A7A\u8868\u793A\u4E0D\u9650\u5236\u4F7F\u7528\u6B21\u6578"
        },
        "limitUseWithUser": {
          "label": "\u6BCF\u500B\u7528\u6236\u53EF\u4F7F\u7528\u6B21\u6578",
          "placeholder": "\u9650\u5236\u6BCF\u500B\u7528\u6236\u53EF\u4F7F\u7528\u6B21\u6578\uFF0C\u7559\u7A7A\u5247\u4E0D\u9650\u5236",
          "description": "\u9650\u5236\u6BCF\u500B\u7528\u6236\u53EF\u4F7F\u7528\u8A72\u512A\u60E0\u5238\u7684\u6B21\u6578\uFF0C\u7559\u7A7A\u8868\u793A\u4E0D\u9650\u5236\u55AE\u7528\u6236\u4F7F\u7528\u6B21\u6578"
        },
        "limitPeriod": {
          "label": "\u6307\u5B9A\u9031\u671F",
          "placeholder": "\u9650\u5236\u6307\u5B9A\u9031\u671F\u53EF\u4EE5\u4F7F\u7528\u512A\u60E0\uFF0C\u7559\u7A7A\u5247\u4E0D\u9650\u5236",
          "description": "\u9078\u64C7\u53EF\u4EE5\u4F7F\u7528\u512A\u60E0\u5238\u7684\u8A02\u95B1\u9031\u671F\uFF0C\u7559\u7A7A\u8868\u793A\u4E0D\u9650\u5236\u4F7F\u7528\u9031\u671F",
          "empty": "\u6C92\u6709\u627E\u5230\u5339\u914D\u7684\u9031\u671F"
        },
        "limitPlan": {
          "label": "\u6307\u5B9A\u8A02\u95B1",
          "placeholder": "\u9650\u5236\u6307\u5B9A\u8A02\u95B1\u53EF\u4EE5\u4F7F\u7528\u512A\u60E0\uFF0C\u7559\u7A7A\u5247\u4E0D\u9650\u5236",
          "description": "\u9078\u64C7\u53EF\u4EE5\u4F7F\u7528\u512A\u60E0\u5238\u7684\u8A02\u95B1\u8A08\u5283\uFF0C\u7559\u7A7A\u8868\u793A\u4E0D\u9650\u5236\u8A08\u5283",
          "empty": "\u6C92\u6709\u627E\u5230\u5339\u914D\u7684\u8A02\u95B1"
        },
        "code": {
          "label": "\u81EA\u5B9A\u7FA9\u512A\u60E0\u78BC",
          "placeholder": "\u81EA\u5B9A\u7FA9\u512A\u60E0\u78BC\uFF0C\u7559\u7A7A\u5247\u81EA\u52D5\u751F\u6210",
          "description": "\u53EF\u4EE5\u81EA\u5B9A\u7FA9\u512A\u60E0\u78BC\uFF0C\u7559\u7A7A\u5247\u7CFB\u7D71\u81EA\u52D5\u751F\u6210"
        },
        "generateCount": {
          "label": "\u6279\u91CF\u751F\u6210\u6578\u91CF",
          "placeholder": "\u6279\u91CF\u751F\u6210\u512A\u60E0\u78BC\u6578\u91CF\uFF0C\u7559\u7A7A\u5247\u751F\u6210\u55AE\u500B",
          "description": "\u6279\u91CF\u751F\u6210\u591A\u500B\u512A\u60E0\u78BC\uFF0C\u7559\u7A7A\u5247\u53EA\u751F\u6210\u55AE\u500B\u512A\u60E0\u78BC"
        },
        "submit": {
          "saving": "\u4FDD\u5B58\u4E2D...",
          "save": "\u4FDD\u5B58"
        },
        "error": {
          "saveFailed": "\u4FDD\u5B58\u512A\u60E0\u5238\u5931\u6557"
        },
        "timeRange": {
          "quickSet": "\u5FEB\u901F\u8A2D\u7F6E",
          "presets": {
            "1week": "1\u5468",
            "2weeks": "2\u5468",
            "1month": "1\u500B\u6708",
            "3months": "3\u500B\u6708",
            "6months": "6\u500B\u6708",
            "1year": "1\u5E74"
          }
        }
      },
      "period": {
        "hourly": "\u5C0F\u6642",
        "daily": "\u5929",
        "monthly": "\u6708",
        "quarterly": "\u5B63\u5EA6",
        "half_yearly": "\u534A\u5E74",
        "yearly": "\u5E74",
        "two_yearly": "\u5169\u5E74",
        "three_yearly": "\u4E09\u5E74",
        "onetime": "\u4E00\u6B21\u6027",
        "reset_traffic": "\u91CD\u7F6E\u6D41\u91CF"
      }
    },
    "route": {
      "title": "\u8DEF\u7531\u7BA1\u7406",
      "description": "\u7BA1\u7406\u6240\u6709\u8DEF\u7531\u7D44\uFF0C\u5305\u62EC\u6DFB\u52A0\u3001\u522A\u9664\u3001\u7DE8\u8F2F\u7B49\u64CD\u4F5C\u3002",
      "columns": {
        "id": "\u7D44ID",
        "remarks": "\u5099\u8A3B",
        "action": "\u52D5\u4F5C",
        "actions": "\u64CD\u4F5C",
        "matchRules": "\u5339\u914D{{count}}\u689D\u898F\u5247",
        "action_value": {
          "title": "\u52D5\u4F5C\u503C",
          "dns": "DNS: {{value}}",
          "proxy": "\u8F49\u767C ({{value}})",
          "block": "\u963B\u6B62\u8A2A\u554F",
          "direct": "\u76F4\u9023"
        }
      },
      "actions": {
        "dns": "\u6307\u5B9ADNS\u670D\u52D9\u5668\u9032\u884C\u89E3\u6790",
        "block": "\u7981\u6B62\u8A2A\u554F",
        "direct": "\u76F4\u9023",
        "proxy": "\u8F49\u767C",
        "short": {
          "dns": "DNS",
          "block": "\u963B\u6B62",
          "direct": "\u76F4\u9023",
          "proxy": "\u8F49\u767C"
        }
      },
      "form": {
        "add": "\u6DFB\u52A0\u8DEF\u7531",
        "edit": "\u7DE8\u8F2F\u8DEF\u7531",
        "create": "\u5275\u5EFA\u8DEF\u7531",
        "remarks": "\u5099\u8A3B",
        "remarksPlaceholder": "\u8ACB\u8F38\u5165\u5099\u8A3B",
        "match": "\u5339\u914D\u898F\u5247",
        "matchPlaceholder": "example.com\n*.example.com",
        "action": "\u52D5\u4F5C",
        "actionPlaceholder": "\u8ACB\u9078\u64C7\u52D5\u4F5C",
        "dns": "DNS\u670D\u52D9\u5668",
        "dnsPlaceholder": "\u8ACB\u8F38\u5165DNS\u670D\u52D9\u5668",
        "proxy": "\u8F49\u767C\u6A19\u7C64 (Outbound Tag)",
        "proxyPlaceholder": "\u8ACB\u8F38\u5165\u8F49\u767C\u6A19\u7C64",
        "cancel": "\u53D6\u6D88",
        "submit": "\u63D0\u4EA4",
        "validation": {
          "remarks": "\u8ACB\u8F38\u5165\u6709\u6548\u7684\u5099\u8A3B"
        }
      },
      "toolbar": {
        "searchPlaceholder": "\u641C\u7D22\u8DEF\u7531...",
        "reset": "\u91CD\u7F6E"
      },
      "messages": {
        "deleteConfirm": "\u78BA\u8A8D\u522A\u9664",
        "deleteDescription": "\u6B64\u64CD\u4F5C\u5C07\u6C38\u4E45\u522A\u9664\u8A72\u8DEF\u7531\u7D44\uFF0C\u522A\u9664\u5F8C\u7121\u6CD5\u6062\u5FA9\u3002\u78BA\u5B9A\u8981\u7E7C\u7E8C\u55CE\uFF1F",
        "deleteButton": "\u522A\u9664",
        "deleteSuccess": "\u522A\u9664\u6210\u529F",
        "createSuccess": "\u5275\u5EFA\u6210\u529F",
        "updateSuccess": "\u66F4\u65B0\u6210\u529F"
      }
    },
    "ticket": {
      "title": "\u5DE5\u55AE\u7BA1\u7406",
      "description": "\u5728\u9019\u88E1\u53EF\u4EE5\u67E5\u770B\u7528\u6236\u5DE5\u55AE\uFF0C\u5305\u62EC\u67E5\u770B\u3001\u56DE\u8986\u3001\u95DC\u9589\u7B49\u64CD\u4F5C\u3002",
      "columns": {
        "id": "\u5DE5\u55AE\u865F",
        "select": "\u9078\u64C7",
        "subject": "\u4E3B\u984C",
        "level": "\u512A\u5148\u7D1A",
        "status": "\u72C0\u614B",
        "updated_at": "\u6700\u5F8C\u66F4\u65B0",
        "created_at": "\u5275\u5EFA\u6642\u9593",
        "actions": "\u64CD\u4F5C"
      },
      "status": {
        "closed": "\u5DF2\u95DC\u9589",
        "replied": "\u5DF2\u56DE\u8986",
        "pending": "\u5F85\u56DE\u8986",
        "processing": "\u8655\u7406\u4E2D",
        "unreplied": "\u672A\u56DE\u8986"
      },
      "level": {
        "low": "\u4F4E\u512A\u5148",
        "medium": "\u4E2D\u512A\u5148",
        "high": "\u9AD8\u512A\u5148"
      },
      "filter": {
        "placeholder": "\u641C\u7D22{field}...",
        "no_results": "\u672A\u627E\u5230\u7D50\u679C",
        "selected": "\u5DF2\u9078\u64C7 {count} \u9805",
        "clear": "\u6E05\u9664\u7BE9\u9078"
      },
      "actions": {
        "reply_success": "\u5DF2\u56DE\u8986",
        "view_details": "\u67E5\u770B\u8A73\u60C5",
        "close_ticket": "\u95DC\u9589\u5DE5\u55AE",
        "close_confirm_title": "\u78BA\u8A8D\u95DC\u9589\u5DE5\u55AE",
        "close_confirm_description": "\u78BA\u5B9A\u8981\u95DC\u9589\u9019\u500B\u5DE5\u55AE\u55CE\uFF1F\u95DC\u9589\u5F8C\u6703\u79FB\u5165\u5DF2\u95DC\u9589\u5217\u8868\uFF0C\u4F46\u4ECD\u53EF\u7E7C\u7E8C\u56DE\u8986\u3002",
        "close_confirm_button": "\u78BA\u8A8D\u95DC\u9589",
        "close_success": "\u5DE5\u55AE\u5DF2\u95DC\u9589",
        "view_ticket": "\u67E5\u770B\u5DE5\u55AE",
        "bulk_close": "\u6279\u91CF\u95DC\u9589 ({{count}})",
        "bulk_close_confirm_title": "\u78BA\u8A8D\u6279\u91CF\u95DC\u9589\u5DE5\u55AE",
        "bulk_close_confirm_description": "\u78BA\u5B9A\u8981\u95DC\u9589\u9078\u4E2D\u7684 {{count}} \u500B\u5DE5\u55AE\u55CE\uFF1F\u95DC\u9589\u5F8C\u4ECD\u53EF\u7E7C\u7E8C\u56DE\u8986\u3002",
        "bulk_close_confirm_button": "\u78BA\u8A8D\u95DC\u9589",
        "bulk_close_success": "\u5DF2\u95DC\u9589 {{count}} \u500B\u5DE5\u55AE"
      },
      "detail": {
        "no_messages": "\u66AB\u7121\u6D88\u606F\u8A18\u9304",
        "created_at": "\u5275\u5EFA\u65BC",
        "sender_admin": "\u7BA1\u7406\u54E1",
        "sender_user": "\u7528\u6236",
        "user_info": "\u7528\u6236\u4FE1\u606F",
        "traffic_records": "\u6D41\u91CF\u8A18\u9304",
        "order_records": "\u8A02\u55AE\u8A18\u9304",
        "message": {
          "edit": "\u4FEE\u6539",
          "recall": "\u64A4\u56DE",
          "save": "\u4FDD\u5B58",
          "cancel": "\u53D6\u6D88",
          "edited": "\u5DF2\u7DE8\u8F2F",
          "recalled": "\u8A72\u6D88\u606F\u5DF2\u64A4\u56DE",
          "edit_success": "\u6D88\u606F\u5DF2\u4FEE\u6539",
          "recall_success": "\u6D88\u606F\u5DF2\u64A4\u56DE",
          "recall_confirm_title": "\u78BA\u8A8D\u64A4\u56DE\u6D88\u606F",
          "recall_confirm_description": "\u64A4\u56DE\u5F8C\u7528\u6236\u5C07\u7121\u6CD5\u518D\u770B\u5230\u9019\u689D\u6D88\u606F\uFF0C\u4E14\u7121\u6CD5\u6062\u5FA9\u3002",
          "recall_confirm_button": "\u78BA\u8A8D\u64A4\u56DE"
        },
        "input": {
          "closed_reply_placeholder": "\u5DE5\u55AE\u5DF2\u95DC\u9589\uFF0C\u4ECD\u53EF\u7E7C\u7E8C\u56DE\u8986...",
          "closed_hint": "\u8A72\u5DE5\u55AE\u5DF2\u95DC\u9589\uFF0C\u4F60\u4ECD\u53EF\u4EE5\u7E7C\u7E8C\u56DE\u8986\uFF0C\u65B0\u7684\u6D88\u606F\u6703\u8FFD\u52A0\u5230\u7576\u524D\u5DE5\u55AE\u3002",
          "reply_placeholder": "\u8F38\u5165\u56DE\u8986\u5167\u5BB9...",
          "sending": "\u767C\u9001\u4E2D...",
          "send": "\u767C\u9001",
          "shortcut_hint": "Enter \u767C\u9001 \xB7 Shift + Enter \u63DB\u884C"
        }
      },
      "list": {
        "title": "\u5DE5\u55AE\u5217\u8868",
        "search_placeholder": "\u641C\u7D22\u5DE5\u55AE\u6A19\u984C\u6216\u7528\u6236\u90F5\u7BB1",
        "no_tickets": "\u66AB\u7121\u5DE5\u55AE",
        "no_open_tickets": "\u66AB\u7121\u8655\u7406\u4E2D\u5DE5\u55AE",
        "no_closed_tickets": "\u66AB\u7121\u5DF2\u95DC\u9589\u5DE5\u55AE",
        "no_search_results": "\u672A\u627E\u5230\u5339\u914D\u7684\u5DE5\u55AE",
        "collapse": "\u6536\u8D77\u5217\u8868",
        "expand": "\u5C55\u958B\u5217\u8868"
      }
    },
    "withdrawal": {
      "title": "\u63D0\u73FE\u7BA1\u7406",
      "description": "\u7BA1\u7406\u7528\u6236\u4F63\u91D1\u63D0\u73FE\u7533\u8ACB\uFF0C\u652F\u6301\u78BA\u8A8D\u63D0\u73FE\u548C\u62D2\u7D55\u63D0\u73FE\u64CD\u4F5C\u3002",
      "columns": {
        "id": "\u63D0\u73FE\u55AE\u865F",
        "user": "\u7528\u6236",
        "method": "\u63D0\u73FE\u65B9\u5F0F",
        "account": "\u63D0\u73FE\u5E33\u865F",
        "amount": "\u91D1\u984D",
        "status": "\u72C0\u614B",
        "created_at": "\u5EFA\u7ACB\u6642\u9593",
        "actions": "\u64CD\u4F5C"
      },
      "status": {
        "pending": "\u5F85\u8655\u7406",
        "confirmed": "\u5DF2\u78BA\u8A8D",
        "closed": "\u5DF2\u62D2\u7D55"
      },
      "filter": {
        "all": "\u5168\u90E8"
      },
      "actions": {
        "view": "\u67E5\u770B\u8A73\u60C5",
        "confirm": "\u78BA\u8A8D\u63D0\u73FE",
        "close": "\u95DC\u9589\u63D0\u73FE",
        "user_info": "\u7528\u6236\u8CC7\u8A0A",
        "confirm_title": "\u78BA\u8A8D\u63D0\u73FE",
        "confirm_description": "\u78BA\u8A8D\u5F8C\u5C07\u95DC\u9589\u8A72\u63D0\u73FE\u55AE\uFF0C\u8ACB\u78BA\u4FDD\u6B3E\u9805\u5DF2\u8655\u7406\u3002",
        "confirm_button": "\u78BA\u8A8D\u63D0\u73FE",
        "confirm_success": "\u63D0\u73FE\u5DF2\u78BA\u8A8D",
        "close_confirm_title": "\u95DC\u9589\u63D0\u73FE",
        "close_confirm_description": "\u95DC\u9589\u5F8C\u5C07\u628A\u4F63\u91D1\u8FD4\u9084\u7D66\u7528\u6236\u5E33\u6236\u3002",
        "close_confirm_button": "\u78BA\u8A8D\u95DC\u9589",
        "close_success": "\u63D0\u73FE\u5DF2\u95DC\u9589\uFF0C\u4F63\u91D1\u5DF2\u8FD4\u9084",
        "reply_success": "\u56DE\u8986\u6210\u529F",
        "confirm_desc": "\u78BA\u8A8D\u5F8C\u5C07\u95DC\u9589\u8A72\u63D0\u73FE\u55AE\uFF0C\u8ACB\u78BA\u4FDD\u6B3E\u9805\u5DF2\u8655\u7406\u3002",
        "close_desc": "\u95DC\u9589\u5F8C\u5C07\u628A\u4F63\u91D1\u8FD4\u9084\u7D66\u7528\u6236\u5E33\u6236\uFF0C\u662F\u5426\u7E7C\u7E8C\uFF1F"
      },
      "list": {
        "search_placeholder": "\u641C\u7D22\u63D0\u73FE\u5E33\u865F...",
        "no_withdrawals": "\u66AB\u7121\u63D0\u73FE\u8A18\u9304",
        "no_open_withdrawals": "\u66AB\u7121\u5F85\u8655\u7406\u63D0\u73FE",
        "title": "\u63D0\u73FE\u5217\u8868",
        "collapse": "\u6536\u8D77\u5217\u8868",
        "expand": "\u5C55\u958B\u5217\u8868"
      },
      "detail": {
        "created_at": "\u5EFA\u7ACB\u6642\u9593",
        "user_info": "\u7528\u6236\u8CC7\u8A0A",
        "traffic_records": "\u6D41\u91CF\u8A18\u9304",
        "order_records": "\u8A02\u55AE\u8A18\u9304",
        "no_messages": "\u66AB\u7121\u804A\u5929\u8A18\u9304",
        "operator": "\u64CD\u4F5C\u54E1",
        "remark": "\u5099\u8A3B",
        "remark_placeholder": "\u8F38\u5165\u5099\u8A3B\uFF08\u53EF\u9078\uFF09",
        "sender_user": "\u7528\u6236",
        "sender_admin": "\u7BA1\u7406\u54E1",
        "input": {
          "closed_hint": "\u8A72\u63D0\u73FE\u55AE\u5DF2\u8655\u7406\uFF0C\u7121\u6CD5\u7E7C\u7E8C\u56DE\u8986",
          "reply_placeholder": "\u8F38\u5165\u6D88\u606F...",
          "closed_reply_placeholder": "\u8A72\u63D0\u73FE\u55AE\u5DF2\u8655\u7406",
          "send": "\u767C\u9001",
          "sending": "\u767C\u9001\u4E2D...",
          "shortcut_hint": "\u56DE\u8ECA\u767C\u9001\uFF0CShift+\u56DE\u8ECA\u63DB\u884C"
        }
      }
    },
    "machine": {
      "title": "\u670D\u52D9\u5668\u7BA1\u7406",
      "description": "\u7528\u65BC\u67E5\u770B\u670D\u52D9\u5668\u5065\u5EB7\u3001\u8CA0\u8F09\u8207\u627F\u8F09\u7BC0\u9EDE\uFF0C\u4E26\u5F9E\u904B\u7DAD\u8996\u89D2\u7BA1\u7406 Fboard-Node \u670D\u52D9\u3002",
      "columns": {
        "id": "ID",
        "name": "\u670D\u52D9\u5668\u540D\u7A31",
        "status": "\u72C0\u614B",
        "nodes": "\u7BC0\u9EDE\u6578",
        "nodesHosted": "\u7BC0\u9EDE\u6578",
        "nodesIdle": "\u66AB\u7121\u627F\u8F09",
        "load": "\u8CA0\u8F09",
        "lastSeen": "\u6700\u5F8C\u5FC3\u8DF3",
        "version": "\u7248\u672C",
        "actions": "\u64CD\u4F5C",
        "online": "\u5728\u7DDA",
        "offline": "\u96E2\u7DDA",
        "inactive": "\u5DF2\u7981\u7528",
        "noData": "\u66AB\u7121\u8CA0\u8F09\u6578\u64DA",
        "cpu": "CPU",
        "memory": "\u5167\u5B58",
        "disk": "\u78C1\u76E4",
        "never": "\u5F9E\u672A\u4E0A\u5831",
        "lastReport": "\u8CA0\u8F09\u4E0A\u5831",
        "kernel": "\u5167\u6838",
        "kernelRunning": "\u904B\u884C\u4E2D",
        "kernelStopped": "\u5DF2\u505C\u6B62",
        "kernelPartial": "\u90E8\u5206\u904B\u884C",
        "kernelIdle": "\u7121\u7BC0\u9EDE",
        "kernelUnknown": "\u672A\u77E5",
        "kernelDetail": "{{running}}/{{total}} \u904B\u884C"
      },
      "toolbar": {
        "search": "\u641C\u7D22\u670D\u52D9\u5668\u540D\u7A31\u6216\u5099\u8A3B...",
        "status": "\u72C0\u614B",
        "status_all": "\u5168\u90E8\u72C0\u614B",
        "status_online": "\u5728\u7DDA",
        "status_offline": "\u96E2\u7DDA",
        "status_inactive": "\u5DF2\u7981\u7528",
        "status_high_load": "\u9AD8\u8CA0\u8F09",
        "nodes": "\u7BC0\u9EDE",
        "with_nodes": "\u5DF2\u627F\u8F09\u7BC0\u9EDE",
        "idle_nodes": "\u7A7A\u9592\u670D\u52D9\u5668",
        "high_load": "\u9AD8\u8CA0\u8F09",
        "online_ratio": "\u5728\u7DDA",
        "high_load_count": "\u9AD8\u8CA0\u8F09",
        "tip": "\u9069\u5408\u96C6\u4E2D\u67E5\u770B\u670D\u52D9\u5668\u5728\u7DDA\u60C5\u6CC1\u3001\u627F\u8F09\u7BC0\u9EDE\u6578\u91CF\u8207\u8CC7\u6E90\u58D3\u529B\u3002",
        "reset": "\u91CD\u7F6E",
        "nodesHosted": "\u7BC0\u9EDE",
        "nodesIdle": "\u7A7A\u9592"
      },
      "operations": {
        "upgrade": "\u5347\u7D1A Fboard-Node",
        "ops": "\u5167\u6838\u904B\u7DAD",
        "start": "\u555F\u52D5\u5167\u6838",
        "stop": "\u505C\u6B62\u5167\u6838",
        "reload": "\u91CD\u8F09\u5167\u6838",
        "restart": "\u91CD\u555F\u5167\u6838",
        "upgradeTitle": "\u78BA\u8A8D\u5347\u7D1A\u670D\u52D9\u5668",
        "upgradeDescription": "\u5C07\u5347\u7D1A\u670D\u52D9\u5668\u201C{{name}}\u201D\u4E0A\u7684 Fboard-Node \u670D\u52D9\u3002\u5347\u7D1A\u904E\u7A0B\u4E2D\u8A72\u670D\u52D9\u5668\u4E0A\u7684\u670D\u52D9\u53EF\u80FD\u77ED\u66AB\u4E2D\u65B7\u3002",
        "startTitle": "\u78BA\u8A8D\u555F\u52D5\u5167\u6838",
        "startDescription": "\u5C07\u555F\u52D5\u670D\u52D9\u5668\u201C{{name}}\u201D\u4E0A\u6240\u6709\u7BC0\u9EDE\u7684\u5167\u5D4C xray \u5167\u6838\u3002fboard-node \u9032\u7A0B\u8207 WebSocket \u4FDD\u6301\u904B\u884C\u3002",
        "stopTitle": "\u78BA\u8A8D\u505C\u6B62\u5167\u6838",
        "stopDescription": "\u5C07\u505C\u6B62\u670D\u52D9\u5668\u201C{{name}}\u201D\u4E0A\u6240\u6709\u7BC0\u9EDE\u7684\u5167\u5D4C xray \u5167\u6838\uFF0C\u8A72\u670D\u52D9\u5668\u4E0A\u7684\u4EE3\u7406\u5C07\u4E0D\u53EF\u7528\uFF0C\u76F4\u5230\u518D\u6B21\u555F\u52D5\u3002\u9032\u7A0B\u8207 WebSocket \u4FDD\u6301\u904B\u884C\u3002",
        "reloadTitle": "\u78BA\u8A8D\u91CD\u8F09\u5167\u6838",
        "reloadDescription": "\u5C07\u91CD\u8F09\u670D\u52D9\u5668\u201C{{name}}\u201D\u4E0A\u6240\u6709\u7BC0\u9EDE\u7684\u5167\u5D4C xray \u5167\u6838\u914D\u7F6E\uFF1B\u9032\u7A0B\u8207 WebSocket \u4FDD\u6301\u904B\u884C\u3002",
        "restartTitle": "\u78BA\u8A8D\u91CD\u555F\u5167\u6838",
        "restartDescription": "\u5C07\u5F37\u5236\u91CD\u5EFA\u670D\u52D9\u5668\u201C{{name}}\u201D\u4E0A\u6240\u6709\u7BC0\u9EDE\u7684\u5167\u5D4C xray \u5167\u6838\uFF0C\u4EE3\u7406\u6703\u77ED\u66AB\u4E2D\u65B7\u3002fboard-node \u9032\u7A0B\u8207 WebSocket \u4FDD\u6301\u904B\u884C\u3002",
        "upgradeSubmitted": "\u670D\u52D9\u5668\u201C{{name}}\u201D\u7684\u5347\u7D1A\u4EFB\u52D9\u5DF2\u63D0\u4EA4",
        "startSubmitted": "\u670D\u52D9\u5668\u201C{{name}}\u201D\u7684\u5167\u6838\u555F\u52D5\u4EFB\u52D9\u5DF2\u63D0\u4EA4",
        "stopSubmitted": "\u670D\u52D9\u5668\u201C{{name}}\u201D\u7684\u5167\u6838\u505C\u6B62\u4EFB\u52D9\u5DF2\u63D0\u4EA4",
        "reloadSubmitted": "\u670D\u52D9\u5668\u201C{{name}}\u201D\u7684\u5167\u6838\u91CD\u8F09\u4EFB\u52D9\u5DF2\u63D0\u4EA4",
        "restartSubmitted": "\u670D\u52D9\u5668\u201C{{name}}\u201D\u7684\u5167\u6838\u91CD\u555F\u4EFB\u52D9\u5DF2\u63D0\u4EA4",
        "batchUpgrade": "\u4E00\u9375\u5347\u7D1A\u670D\u52D9\u5668",
        "batchUpgradeTitle": "\u78BA\u8A8D\u6279\u91CF\u5347\u7D1A\u670D\u52D9\u5668",
        "batchUpgradeDescription": "\u5C07\u5C0D\u6240\u6709\u5728\u7DDA\u4E14\u555F\u7528\u7684\u670D\u52D9\u5668\u5404\u63D0\u4EA4\u4E00\u6B21 Fboard-Node \u5347\u7D1A\u4EFB\u52D9\uFF0C\u4E0D\u4F9D\u8CF4\u670D\u52D9\u5668\u627F\u8F09\u7684\u7BC0\u9EDE\u6578\u91CF\u3002",
        "batchUpgradeSubmitted": "\u5DF2\u63D0\u4EA4 {{submitted}} \u81FA\u670D\u52D9\u5668\uFF1B\u8DF3\u904E {{inactive}} \u81FA\u7981\u7528\u3001{{offline}} \u81FA\u96E2\u7DDA\u670D\u52D9\u5668"
      },
      "overview": {
        "total": "\u670D\u52D9\u5668\u7E3D\u6578",
        "total_hint": "\u5171\u627F\u8F09 {{count}} \u500B\u7BC0\u9EDE",
        "online": "\u5728\u7DDA\u670D\u52D9\u5668",
        "online_hint": "\u6700\u8FD1 5 \u5206\u9418\u5167\u6B63\u5E38\u5FC3\u8DF3",
        "offline": "\u96E2\u7DDA/\u5931\u806F",
        "offline_hint": "\u9700\u8981\u6AA2\u67E5\u5FC3\u8DF3\u6216\u7BC0\u9EDE\u4EE3\u7406",
        "high_load": "\u9AD8\u8CA0\u8F09",
        "high_load_hint": "CPU\u3001\u5167\u5B58\u6216\u78C1\u76E4\u63A5\u8FD1\u95BE\u503C",
        "nodes_suffix": "\u7BC0\u9EDE",
        "attention": "\u9700\u95DC\u6CE8",
        "stable": "\u7A69\u5B9A",
        "needs_review": "\u5EFA\u8B70\u6AA2\u67E5",
        "normal": "\u6B63\u5E38"
      },
      "form": {
        "add": "\u6DFB\u52A0\u670D\u52D9\u5668",
        "create": "\u65B0\u5EFA\u670D\u52D9\u5668",
        "edit": "\u7DE8\u8F2F\u670D\u52D9\u5668",
        "createDescription": "\u7576\u4F60\u5E0C\u671B\u4E00\u81FA\u670D\u52D9\u5668\u627F\u8F09\u591A\u500B\u7BC0\u9EDE\u6642\uFF0C\u518D\u5275\u5EFA\u670D\u52D9\u5668\u8A18\u9304\u3002",
        "editDescription": "\u4FEE\u6539\u670D\u52D9\u5668\u540D\u7A31\u3001\u5099\u8A3B\u6216\u555F\u7528\u72C0\u614B\u3002",
        "name": "\u670D\u52D9\u5668\u540D\u7A31",
        "namePlaceholder": "\u4F8B\u5982 HK-01",
        "nameError": "\u8ACB\u8F38\u5165\u670D\u52D9\u5668\u540D\u7A31",
        "notes": "\u5099\u8A3B",
        "notesPlaceholder": "\u95DC\u65BC\u6B64\u670D\u52D9\u5668\u7684\u53EF\u9078\u5099\u8A3B",
        "isActive": "\u555F\u7528\u670D\u52D9\u5668",
        "isActiveDescription": "\u7981\u7528\u5F8C Fboard-Node \u5C07\u4E0D\u518D\u4F7F\u7528\u6B64\u670D\u52D9\u5668\u3002",
        "cancel": "\u53D6\u6D88",
        "submit": "\u63D0\u4EA4",
        "update": "\u66F4\u65B0"
      },
      "token": {
        "title": "\u670D\u52D9\u5668 Token",
        "description": "\u6B64 Token \u7528\u65BC Fboard-Node \u5411\u9762\u677F\u8A8D\u8B49\uFF0C\u8ACB\u59A5\u5584\u4FDD\u7BA1\u3002",
        "show": "\u67E5\u770B Token",
        "hide": "\u96B1\u85CF Token",
        "reset": "\u91CD\u7F6E Token",
        "resetConfirm": "\u78BA\u8A8D\u91CD\u7F6E Token\uFF1F",
        "resetDescription": "\u820A Token \u5C07\u7ACB\u5373\u5931\u6548\uFF0CFboard-Node \u9700\u8981\u91CD\u65B0\u914D\u7F6E\u65B0 Token\u3002",
        "copy": "\u8907\u88FD",
        "copied": "Token \u5DF2\u8907\u88FD\u5230\u526A\u8CBC\u677F",
        "copiedInline": "\u5DF2\u8907\u88FD!",
        "copyFailed": "\u8907\u88FD\u5931\u6557\uFF0C\u8ACB\u624B\u52D5\u8907\u88FD",
        "autoHide": "{{time}} \u5F8C\u81EA\u52D5\u96B1\u85CF",
        "resetSuccess": "Token \u5DF2\u91CD\u7F6E",
        "createdHint": "Token \u5DF2\u751F\u6210\uFF0C\u5F8C\u7E8C\u53EF\u5728\u670D\u52D9\u5668\u8A73\u60C5\u9801\u4E2D\u67E5\u770B\u3002"
      },
      "install": {
        "title": "\u5B89\u88DD Fboard-Node",
        "description": "\u5728\u76EE\u6A19\u670D\u52D9\u5668\u4E0A\u57F7\u884C\u6B64\u547D\u4EE4\uFF0C\u5373\u53EF\u7528 machine mode \u5B89\u88DD Fboard-Node \u4E26\u63A5\u5165\u7576\u524D\u670D\u52D9\u5668\u8A18\u9304\u3002",
        "copy": "\u8907\u88FD\u5B89\u88DD\u547D\u4EE4",
        "copied": "\u5B89\u88DD\u547D\u4EE4\u5DF2\u8907\u88FD",
        "copiedInline": "\u5DF2\u8907\u88FD!",
        "copyFailed": "\u8907\u88FD\u5931\u6557",
        "loading": "\u6B63\u5728\u751F\u6210\u547D\u4EE4...",
        "hint": "\u9700\u8981 root \u6216 sudo \u6B0A\u9650\uFF0C\u4E14\u76EE\u6A19\u670D\u52D9\u5668\u9700\u70BA\u652F\u6301 systemd \u7684 Linux\u3002"
      },
      "logs": {
        "title": "\u904B\u884C\u65E5\u8A8C",
        "description": "\u4F86\u81EA Fboard-Node \u9032\u7A0B\u5167\u5B58\u4E2D\u7684\u6700\u8FD1\u65E5\u8A8C\uFF08\u6700\u591A\u7D04 1000 \u884C\uFF09\u3002",
        "refresh": "\u5237\u65B0\u65E5\u8A8C",
        "copy": "\u8907\u88FD\u65E5\u8A8C",
        "copied": "\u65E5\u8A8C\u5DF2\u8907\u88FD",
        "loading": "\u6B63\u5728\u62C9\u53D6\u65E5\u8A8C...",
        "empty": "\u66AB\u7121\u65E5\u8A8C",
        "offline": "\u670D\u52D9\u5668\u96E2\u7DDA\uFF0C\u7121\u6CD5\u62C9\u53D6\u5BE6\u6642\u65E5\u8A8C",
        "stale": "\u986F\u793A\u7DE9\u5B58\u65E5\u8A8C\uFF08\u53EF\u80FD\u4E0D\u662F\u6700\u65B0\uFF09",
        "timeout": "\u7B49\u5F85\u7BC0\u9EDE\u97FF\u61C9\u8D85\u6642",
        "updatedAt": "\u66F4\u65B0\u65BC {{time}}",
        "lineCount": "{{count}} \u884C",
        "autoScroll": "\u81EA\u52D5\u6EFE\u52D5\u5230\u5E95\u90E8",
        "fetchFailed": "\u62C9\u53D6\u65E5\u8A8C\u5931\u6557"
      },
      "detail": {
        "title": "\u670D\u52D9\u5668\u8A73\u60C5",
        "info": "\u670D\u52D9\u5668\u4FE1\u606F",
        "associatedNodes": "\u95DC\u806F\u7BC0\u9EDE",
        "noNodes": "\u66AB\u7121\u7D81\u5B9A\u7BC0\u9EDE\u3002",
        "nodeId": "ID",
        "nodeName": "\u540D\u7A31",
        "nodeType": "\u985E\u578B",
        "nodeHost": "\u5730\u5740",
        "nodePort": "\u7AEF\u53E3",
        "nodeShow": "\u53EF\u898B",
        "nodeEnabled": "\u5DF2\u6FC0\u6D3B",
        "loadTrend": "\u8CA0\u8F09\u8DA8\u52E2",
        "networkTrend": "\u7DB2\u7D61\u901F\u7387",
        "noHistory": "\u66AB\u7121\u6B77\u53F2\u8CA0\u8F09\u6578\u64DA",
        "openNodeManage": "\u524D\u5F80\u7BC0\u9EDE\u7BA1\u7406",
        "addNodeToServer": "\u65B0\u589E\u7BC0\u9EDE\u5230\u6B64\u670D\u52D9\u5668",
        "nodeCount": "{{count}} \u500B\u7BC0\u9EDE",
        "nodeEnabledCount": "{{count}} \u500B\u5DF2\u6FC0\u6D3B",
        "toggleEnabledError": "\u5207\u63DB\u7BC0\u9EDE\u72C0\u614B\u5931\u6557",
        "bindExistingButton": "\u95DC\u806F\u5DF2\u6709\u7BC0\u9EDE",
        "bindExistingTitle": "\u95DC\u806F\u5DF2\u6709\u7BC0\u9EDE",
        "bindExistingDescription": "\u9078\u64C7\u8981\u95DC\u806F\u5230\u300C{{name}}\u300D\u7684\u7BC0\u9EDE",
        "bindSearchPlaceholder": "\u641C\u7D22\u7BC0\u9EDE\u540D\u7A31\u3001\u5730\u5740\u3001\u985E\u578B...",
        "bindTypeAll": "\u5168\u90E8\u985E\u578B",
        "noUnboundNodes": "\u6C92\u6709\u672A\u7D81\u5B9A\u7684\u7BC0\u9EDE",
        "noSearchResults": "\u6C92\u6709\u5339\u914D\u7684\u7BC0\u9EDE",
        "selectAll": "\u5168\u9078\uFF08\u5171 {{count}} \u500B\uFF09",
        "selectedCount": "\u5DF2\u9078 {{count}} \u500B",
        "bindConfirm": "\u95DC\u806F {{count}} \u500B\u7BC0\u9EDE",
        "binding": "\u95DC\u806F\u4E2D...",
        "bindSuccess": "\u6210\u529F\u5C07 {{count}} \u500B\u7BC0\u9EDE\u95DC\u806F\u5230\u300C{{name}}\u300D",
        "bindFailed": "\u95DC\u806F\u5931\u6557",
        "unbindNode": "\u53D6\u6D88\u95DC\u806F",
        "unbindSuccess": "\u5DF2\u53D6\u6D88\u300C{{name}}\u300D\u7684\u95DC\u806F",
        "unbindFailed": "\u53D6\u6D88\u95DC\u806F\u5931\u6557",
        "unbindConfirmTitle": "\u78BA\u8A8D\u53D6\u6D88\u95DC\u806F",
        "unbindConfirmDescription": "\u5C07\u628A\u7BC0\u9EDE\u300C{{name}}\u300D\u5F9E\u7576\u524D\u670D\u52D9\u5668\u4E0A\u89E3\u7D81\uFF08\u7BC0\u9EDE\u672C\u8EAB\u4E0D\u6703\u88AB\u522A\u9664\uFF09\u3002",
        "cancel": "\u53D6\u6D88"
      },
      "messages": {
        "createSuccess": "\u670D\u52D9\u5668\u5275\u5EFA\u6210\u529F",
        "updateSuccess": "\u670D\u52D9\u5668\u66F4\u65B0\u6210\u529F",
        "deleteConfirm": "\u78BA\u8A8D\u522A\u9664\u670D\u52D9\u5668\uFF1F",
        "deleteDescription": "\u95DC\u806F\u7BC0\u9EDE\u5C07\u81EA\u52D5\u89E3\u7D81\uFF08\u4E0D\u6703\u88AB\u522A\u9664\uFF09\uFF0C\u6B64\u64CD\u4F5C\u4E0D\u53EF\u64A4\u92B7\u3002",
        "deleteButton": "\u522A\u9664",
        "deleteSuccess": "\u670D\u52D9\u5668\u522A\u9664\u6210\u529F",
        "deleteFailed": "\u522A\u9664\u670D\u52D9\u5668\u5931\u6557",
        "saveFailed": "\u4FDD\u5B58\u670D\u52D9\u5668\u5931\u6557",
        "tokenFetchFailed": "\u7372\u53D6\u4EE4\u724C\u5931\u6557",
        "tokenResetFailed": "\u91CD\u7F6E\u4EE4\u724C\u5931\u6557"
      },
      "nodeForm": {
        "machineId": "\u7D81\u5B9A\u670D\u52D9\u5668",
        "machineIdPlaceholder": "\u9078\u64C7\u4E00\u81FA\u670D\u52D9\u5668\uFF08\u53EF\u9078\uFF09",
        "machineIdNone": "\u7368\u7ACB\u90E8\u7F72",
        "enabled": "\u5728\u670D\u52D9\u5668\u4E0A\u6FC0\u6D3B",
        "enabledDescription": "\u7BC0\u9EDE\u662F\u5426\u5728\u6240\u9078\u670D\u52D9\u5668\u4E0A\u555F\u7528\u904B\u884C"
      },
      "nodesStatus": {
        "toggleHint": "\u5728\u7BC0\u9EDE\u7BA1\u7406\u4E2D\u4FEE\u6539\u72C0\u614B"
      }
    },
    "search": {
      "placeholder": "\u641C\u7D22\u83DC\u55AE\u548C\u529F\u80FD...",
      "title": "\u83DC\u55AE\u5C0E\u822A",
      "noResults": "\u672A\u627E\u5230\u7D50\u679C",
      "shortcut": {
        "label": "\u641C\u7D22",
        "key": "\u2318K"
      }
    },
    "knowledge": {
      "title": "\u77E5\u8B58\u5EAB\u7BA1\u7406",
      "description": "\u5728\u9019\u88E1\u53EF\u4EE5\u914D\u7F6E\u77E5\u8B58\u5EAB\uFF0C\u5305\u62EC\u6DFB\u52A0\u3001\u522A\u9664\u3001\u7DE8\u8F2F\u7B49\u64CD\u4F5C\u3002",
      "columns": {
        "id": "ID",
        "status": "\u72C0\u614B",
        "title": "\u6A19\u984C",
        "category": "\u5206\u985E",
        "actions": "\u64CD\u4F5C"
      },
      "form": {
        "add": "\u6DFB\u52A0\u77E5\u8B58",
        "edit": "\u7DE8\u8F2F\u77E5\u8B58",
        "title": "\u6A19\u984C",
        "titlePlaceholder": "\u8ACB\u8F38\u5165\u77E5\u8B58\u6A19\u984C",
        "category": "\u5206\u985E",
        "categoryPlaceholder": "\u8ACB\u8F38\u5165\u5206\u985E\uFF0C\u5206\u985E\u5C07\u6703\u81EA\u52D5\u6B78\u985E",
        "language": "\u8A9E\u8A00",
        "languagePlaceholder": "\u8ACB\u9078\u64C7\u8A9E\u8A00",
        "content": "\u5167\u5BB9",
        "show": "\u986F\u793A",
        "cancel": "\u53D6\u6D88",
        "submit": "\u63D0\u4EA4"
      },
      "languages": {
        "en-US": "English",
        "ja-JP": "\u65E5\u672C\u8A9E",
        "ko-KR": "\uD55C\uAD6D\uC5B4",
        "vi-VN": "Ti\u1EBFng Vi\u1EC7t",
        "zh-CN": "\u7C21\u9AD4\u4E2D\u6587",
        "zh-TW": "\u7E41\u9AD4\u4E2D\u6587",
        "ru-RU": "\u0420\u0443\u0441\u0441\u043A\u0438\u0439"
      },
      "messages": {
        "deleteConfirm": "\u78BA\u8A8D\u522A\u9664",
        "deleteDescription": "\u6B64\u64CD\u4F5C\u5C07\u6C38\u4E45\u522A\u9664\u8A72\u77E5\u8B58\u5EAB\u8A18\u9304\uFF0C\u522A\u9664\u5F8C\u7121\u6CD5\u6062\u5FA9\u3002\u78BA\u5B9A\u8981\u7E7C\u7E8C\u55CE\uFF1F",
        "deleteButton": "\u522A\u9664",
        "operationSuccess": "\u64CD\u4F5C\u6210\u529F",
        "loadError": "\u52A0\u8F09\u5931\u6557"
      },
      "toolbar": {
        "searchPlaceholder": "\u641C\u7D22\u77E5\u8B58...",
        "reset": "\u91CD\u7F6E",
        "sortModeHint": "\u62D6\u62FD\u77E5\u8B58\u689D\u76EE\u9032\u884C\u6392\u5E8F\uFF0C\u5B8C\u6210\u5F8C\u9EDE\u64CA\u4FDD\u5B58",
        "editSort": "\u7DE8\u8F2F\u6392\u5E8F",
        "saveSort": "\u4FDD\u5B58\u6392\u5E8F",
        "allCategories": "\u5168\u90E8\u5206\u985E"
      }
    },
    "common": {
      "all": "\u5168\u90E8",
      "clear": "\u6E05\u9664",
      "selectAll": "\u5168\u9078",
      "loading": "\u52A0\u8F09\u4E2D...",
      "error": "\u932F\u8AA4",
      "success": "\u6210\u529F",
      "save": "\u4FDD\u5B58",
      "cancel": "\u53D6\u6D88",
      "confirm": "\u78BA\u8A8D",
      "close": "\u95DC\u9589",
      "delete": {
        "success": "\u522A\u9664\u6210\u529F",
        "failed": "\u522A\u9664\u5931\u6557"
      },
      "edit": "\u7DE8\u8F2F",
      "view": "\u67E5\u770B",
      "toggleNavigation": "\u5207\u63DB\u5C0E\u822A",
      "toggleSidebar": "\u5207\u63DB\u5074\u908A\u6B04",
      "search": "\u641C\u7D22...",
      "noMatch": "\u7121\u5339\u914D\u9078\u9805",
      "selectField": "\u9078\u64C7{{name}}",
      "inputField": "\u8F38\u5165{{name}}",
      "pageNotImplemented": "\u8A72\u9801\u9762\u5F85\u5BE6\u73FE",
      "theme": {
        "label": "\u4E3B\u984C",
        "light": "\u6DFA\u8272",
        "dark": "\u6DF1\u8272",
        "system": "\u8DDF\u96A8\u7CFB\u7D71"
      },
      "user": "\u7528\u6236",
      "defaultEmail": "user@example.com",
      "settings": "\u8A2D\u7F6E",
      "logout": "\u9000\u51FA\u767B\u9304",
      "copy": {
        "success": "\u8907\u88FD\u6210\u529F",
        "failed": "\u8907\u88FD\u5931\u6557",
        "error": "\u8907\u88FD\u5931\u6557",
        "errorLog": "\u8907\u88FD\u5230\u526A\u8CBC\u677F\u6642\u51FA\u932F"
      },
      "submit": "\u63D0\u4EA4",
      "saving": "\u4FDD\u5B58\u4E2D...",
      "table": {
        "noData": "\u66AB\u7121\u6578\u64DA",
        "pagination": {
          "selected": "\u5DF2\u9078\u64C7 {{selected}} \u9805\uFF0C\u5171 {{total}} \u9805",
          "itemsPerPage": "\u6BCF\u9801",
          "page": "\u7B2C",
          "pageOf": "\u7B2C {{page}}/{{total}} \u9801",
          "range": "\u7B2C {{from}}\u2013{{to}} \u689D\uFF0C\u5171 {{total}} \u689D",
          "firstPage": "\u8DF3\u8F49\u5230\u7B2C\u4E00\u9801",
          "previousPage": "\u4E0A\u4E00\u9801",
          "nextPage": "\u4E0B\u4E00\u9801",
          "lastPage": "\u8DF3\u8F49\u5230\u6700\u5F8C\u4E00\u9801"
        },
        "viewOptions": {
          "button": "\u986F\u793A\u5217",
          "label": "\u5207\u63DB\u986F\u793A\u5217"
        }
      },
      "update": {
        "title": "\u7CFB\u7D71\u66F4\u65B0",
        "newVersion": "\u767C\u73FE\u65B0\u7248\u672C",
        "currentVersion": "\u7576\u524D\u7248\u672C",
        "latestVersion": "\u6700\u65B0\u7248\u672C",
        "updateLater": "\u7A0D\u5F8C\u66F4\u65B0",
        "updateNow": "\u7ACB\u5373\u66F4\u65B0",
        "updating": "\u66F4\u65B0\u4E2D...",
        "updateSuccess": "\u66F4\u65B0\u6210\u529F\uFF0C\u7CFB\u7D71\u5C07\u5728\u7A0D\u5F8C\u81EA\u52D5\u91CD\u555F",
        "updateFailed": "\u66F4\u65B0\u5931\u6557\uFF0C\u8ACB\u7A0D\u5F8C\u91CD\u8A66"
      },
      "time": {
        "day": "\u5929",
        "hour": "\u5C0F\u6642"
      },
      "reset": "\u91CD\u7F6E",
      "export": "\u5C0E\u51FA",
      "currency": {
        "yuan": "\u5143"
      },
      "http": {
        "notLoggedIn": "\u672A\u767B\u9304",
        "unknownError": "\u672A\u77E5\u932F\u8AA4",
        "loginExpired": "\u767B\u9304\u5DF2\u904E\u671F",
        "loginExpiredRelogin": "\u767B\u9304\u5DF2\u904E\u671F\uFF0C\u8ACB\u91CD\u65B0\u767B\u9304",
        "unauthorized": "\u672A\u6388\u6B0A\uFF0C\u8ACB\u91CD\u65B0\u767B\u9304",
        "invalidCredentials": "\u90F5\u7BB1\u6216\u5BC6\u78BC\u932F\u8AA4",
        "invalidData": "\u63D0\u4EA4\u7684\u6578\u64DA\u6709\u8AA4\uFF0C\u8ACB\u6AA2\u67E5\u8F38\u5165",
        "requestFailed": "\u8ACB\u6C42\u5931\u6557",
        "networkError": "\u7DB2\u7D61\u932F\u8AA4",
        "noPermission": "\u6C92\u6709\u6B0A\u9650",
        "notFound": "\u8CC7\u6E90\u6216\u63A5\u53E3\u4E0D\u5B58\u5728",
        "unknownException": "\u672A\u77E5\u7570\u5E38",
        "success": "\u64CD\u4F5C\u6210\u529F"
      },
      "add": "\u6DFB\u52A0",
      "refresh": "\u5237\u65B0",
      "sort": {
        "edit": "\u7DE8\u8F2F\u6392\u5E8F",
        "done": "\u5B8C\u6210\u6392\u5E8F"
      },
      "actions": "\u64CD\u4F5C",
      "start": "\u8D77",
      "end": "\u6B62"
    },
    "plugin": {
      "title": "\u63D2\u4EF6\u7BA1\u7406",
      "description": "\u7BA1\u7406\u548C\u914D\u7F6E\u7CFB\u7D71\u63D2\u4EF6",
      "search": {
        "placeholder": "\u641C\u7D22\u63D2\u4EF6\u540D\u7A31\u6216\u63CF\u8FF0..."
      },
      "type": {
        "placeholder": "\u9078\u64C7\u63D2\u4EF6\u985E\u578B",
        "all": "\u5168\u90E8\u985E\u578B"
      },
      "tabs": {
        "all": "\u6240\u6709\u63D2\u4EF6",
        "installed": "\u5DF2\u5B89\u88DD",
        "available": "\u53EF\u7528"
      },
      "status": {
        "enabled": "\u5DF2\u555F\u7528",
        "disabled": "\u5DF2\u7981\u7528",
        "not_installed": "\u672A\u5B89\u88DD",
        "protected": "\u53D7\u4FDD\u8B77",
        "filter_placeholder": "\u5B89\u88DD\u72C0\u614B",
        "all": "\u5168\u90E8\u72C0\u614B",
        "installed": "\u5DF2\u5B89\u88DD",
        "available": "\u53EF\u5B89\u88DD"
      },
      "button": {
        "install": "\u5B89\u88DD",
        "upgrade": "\u5347\u7D1A",
        "config": "\u914D\u7F6E",
        "enable": "\u555F\u7528",
        "disable": "\u7981\u7528",
        "uninstall": "\u5378\u8F09",
        "readme": "\u67E5\u770B\u6587\u6A94",
        "menuDemo": "\u83DC\u55AE\u6F14\u793A"
      },
      "upload": {
        "button": "\u4E0A\u50B3\u63D2\u4EF6",
        "title": "\u4E0A\u50B3\u63D2\u4EF6",
        "description": "\u4E0A\u50B3\u63D2\u4EF6\u5305 (.zip)",
        "dragText": "\u62D6\u62FD\u63D2\u4EF6\u5305\u5230\u6B64\u8655\uFF0C\u6216",
        "clickText": "\u700F\u89BD",
        "supportText": "\u50C5\u652F\u6301 .zip \u683C\u5F0F\u6587\u4EF6",
        "uploading": "\u4E0A\u50B3\u4E2D...",
        "error": {
          "format": "\u50C5\u652F\u6301 .zip \u683C\u5F0F\u6587\u4EF6"
        }
      },
      "delete": {
        "title": "\u522A\u9664\u63D2\u4EF6",
        "description": "\u78BA\u5B9A\u8981\u522A\u9664\u6B64\u63D2\u4EF6\u55CE\uFF1F\u6B64\u64CD\u4F5C\u7121\u6CD5\u64A4\u92B7\u3002",
        "button": "\u522A\u9664"
      },
      "uninstall": {
        "title": "\u5378\u8F09\u63D2\u4EF6",
        "description": "\u78BA\u5B9A\u8981\u5378\u8F09\u6B64\u63D2\u4EF6\u55CE\uFF1F\u5378\u8F09\u5F8C\u63D2\u4EF6\u6578\u64DA\u5C07\u88AB\u6E05\u9664\u3002",
        "button": "\u5378\u8F09"
      },
      "upgrade": {
        "title": "\u5347\u7D1A\u63D2\u4EF6",
        "description": "\u78BA\u5B9A\u8981\u5347\u7D1A\u6B64\u63D2\u4EF6\u55CE\uFF1F\u5347\u7D1A\u904E\u7A0B\u4E2D\u63D2\u4EF6\u5C07\u66AB\u6642\u4E0D\u53EF\u7528\u3002",
        "button": "\u5347\u7D1A"
      },
      "config": {
        "title": "\u914D\u7F6E",
        "description": "\u4FEE\u6539\u63D2\u4EF6\u914D\u7F6E",
        "save": "\u4FDD\u5B58",
        "cancel": "\u53D6\u6D88",
        "noConfigs": "\u8A72\u63D2\u4EF6\u6C92\u6709\u53EF\u914D\u7F6E\u7684\u9078\u9805",
        "actions": "\u64CD\u4F5C",
        "selectPlan": "\u8ACB\u9078\u64C7\u5957\u9910",
        "noPlan": "\u4E0D\u9078\u64C7\u5957\u9910"
      },
      "readme": {
        "title": "\u63D2\u4EF6\u6587\u6A94",
        "empty": "\u66AB\u7121\u6587\u6A94"
      },
      "author": "\u4F5C\u8005",
      "messages": {
        "installSuccess": "\u63D2\u4EF6\u5B89\u88DD\u6210\u529F",
        "installError": "\u63D2\u4EF6\u5B89\u88DD\u5931\u6557",
        "upgradeSuccess": "\u63D2\u4EF6\u5347\u7D1A\u6210\u529F",
        "upgradeError": "\u63D2\u4EF6\u5347\u7D1A\u5931\u6557",
        "uninstallSuccess": "\u63D2\u4EF6\u5378\u8F09\u6210\u529F",
        "uninstallError": "\u63D2\u4EF6\u5378\u8F09\u5931\u6557",
        "enableSuccess": "\u63D2\u4EF6\u555F\u7528\u6210\u529F",
        "enableError": "\u63D2\u4EF6\u555F\u7528\u5931\u6557",
        "disableSuccess": "\u63D2\u4EF6\u7981\u7528\u6210\u529F",
        "disableError": "\u63D2\u4EF6\u7981\u7528\u5931\u6557",
        "configLoadError": "\u52A0\u8F09\u63D2\u4EF6\u914D\u7F6E\u5931\u6557",
        "configSaveSuccess": "\u914D\u7F6E\u4FDD\u5B58\u6210\u529F",
        "configSaveError": "\u914D\u7F6E\u4FDD\u5B58\u5931\u6557",
        "uploadSuccess": "\u63D2\u4EF6\u4E0A\u50B3\u6210\u529F",
        "uploadError": "\u63D2\u4EF6\u4E0A\u50B3\u5931\u6557",
        "deleteSuccess": "\u63D2\u4EF6\u522A\u9664\u6210\u529F",
        "deleteError": "\u63D2\u4EF6\u522A\u9664\u5931\u6557",
        "actionSuccess": "\u57F7\u884C\u6210\u529F",
        "actionError": "\u57F7\u884C\u5931\u6557",
        "actionLabel": "\u52D5\u4F5C",
        "actionSuccessWithLabel": "{{label}} \u57F7\u884C\u6210\u529F",
        "actionErrorWithLabel": "{{label}} \u57F7\u884C\u5931\u6557",
        "invalidJson": "{{field}} \u4E0D\u662F\u6709\u6548\u7684 JSON"
      },
      "staticFiles": {
        "title": "HTML \u975C\u614B\u6587\u4EF6",
        "previewTitle": "\u63D2\u4EF6 HTML \u9810\u89BD",
        "backToList": "\u8FD4\u56DE\u5217\u8868",
        "openInNewTab": "\u65B0\u6A19\u7C64\u9801\u6253\u958B",
        "empty": "\u66AB\u7121\u975C\u614B\u6587\u4EF6"
      },
      "noPlugins": "\u66AB\u7121\u63D2\u4EF6",
      "toolbar": {
        "search": "\u641C\u7D22\u63D2\u4EF6..."
      }
    },
    "dashboard": {
      "title": "\u5100\u8868\u76E4",
      "stats": {
        "newUsers": "\u65B0\u7528\u6236",
        "totalScore": "\u7E3D\u7A4D\u5206",
        "monthlyUpload": "\u6708\u4E0A\u50B3",
        "vsLastMonth": "\u5C0D\u6BD4\u4E0A\u6708",
        "vsYesterday": "\u5C0D\u6BD4\u6628\u65E5",
        "todayIncome": "\u4ECA\u65E5\u6536\u5165",
        "monthlyIncome": "\u6708\u6536\u5165",
        "totalIncome": "\u7E3D\u6536\u5165",
        "totalUsers": "\u7E3D\u7528\u6236",
        "activeUsers": "\u6D3B\u8E8D\u7528\u6236: {{count}}",
        "totalOrders": "\u7E3D\u8A02\u55AE",
        "revenue": "\u6536\u5165",
        "todayRegistered": "\u4ECA\u65E5\u8A3B\u518A",
        "monthlyRegistered": "\u6708\u8A3B\u518A",
        "onlineUsers": "\u5728\u7DDA\u7528\u6236",
        "pendingTickets": "\u5F85\u8655\u7406\u5DE5\u55AE",
        "hasPendingTickets": "\u6709\u5DE5\u55AE\u9700\u8981\u8655\u7406",
        "noPendingTickets": "\u7121\u5F85\u8655\u7406\u5DE5\u55AE",
        "pendingCommission": "\u5F85\u8655\u7406\u4F63\u91D1",
        "hasPendingCommission": "\u6709\u4F63\u91D1\u9700\u8981\u78BA\u8A8D",
        "noPendingCommission": "\u7121\u5F85\u8655\u7406\u4F63\u91D1",
        "monthlyNewUsers": "\u6708\u65B0\u589E\u7528\u6236",
        "monthlyDownload": "\u6708\u4E0B\u8F09",
        "todayTraffic": "\u4ECA\u65E5: {{value}}",
        "activeUserTrend": "\u6D3B\u8E8D\u7528\u6236\u8DA8\u52E2",
        "realtimeUsers": "\u5BE6\u6642\u7528\u6236",
        "todayPeak": "\u4ECA\u65E5\u5CF0\u503C",
        "vsLastWeek": "\u5C0D\u6BD4\u4E0A\u9031"
      },
      "trafficRank": {
        "nodeTrafficRank": "\u7BC0\u9EDE\u6D41\u91CF\u6392\u884C",
        "userTrafficRank": "\u7528\u6236\u6D41\u91CF\u6392\u884C",
        "today": "\u4ECA\u5929",
        "last7days": "\u6700\u8FD17\u5929",
        "last30days": "\u6700\u8FD130\u5929",
        "customRange": "\u81EA\u5B9A\u7FA9\u7BC4\u570D",
        "selectTimeRange": "\u9078\u64C7\u6642\u9593\u7BC4\u570D",
        "selectDateRange": "\u9078\u64C7\u65E5\u671F\u7BC4\u570D",
        "currentTraffic": "\u7576\u524D\u6D41\u91CF",
        "previousTraffic": "\u4E0A\u671F\u6D41\u91CF",
        "changeRate": "\u8B8A\u5316\u7387",
        "recordTime": "\u8A18\u9304\u6642\u9593"
      },
      "overview": {
        "title": "\u6536\u5165\u6982\u89BD",
        "thisMonth": "\u672C\u6708",
        "lastMonth": "\u4E0A\u6708",
        "to": "\u81F3",
        "selectTimeRange": "\u9078\u64C7\u7BC4\u570D",
        "selectDate": "\u9078\u64C7\u65E5\u671F",
        "last7Days": "\u6700\u8FD17\u5929",
        "last30Days": "\u6700\u8FD130\u5929",
        "last90Days": "\u6700\u8FD190\u5929",
        "last180Days": "\u6700\u8FD1180\u5929",
        "lastYear": "\u6700\u8FD1\u4E00\u5E74",
        "customRange": "\u81EA\u5B9A\u7FA9\u7BC4\u570D",
        "amount": "\u91D1\u984D",
        "count": "\u6578\u91CF",
        "transactions": "{{count}} \u7B46\u4EA4\u6613",
        "orderAmount": "\u8A02\u55AE\u91D1\u984D",
        "commissionAmount": "\u4F63\u91D1\u91D1\u984D",
        "orderCount": "\u8A02\u55AE\u6578\u91CF",
        "commissionCount": "\u4F63\u91D1\u6578\u91CF",
        "totalIncome": "\u7E3D\u6536\u5165",
        "totalCommission": "\u7E3D\u4F63\u91D1",
        "totalTransactions": "\u5171 {{count}} \u7B46\u4EA4\u6613",
        "avgOrderAmount": "\u5E73\u5747\u8A02\u55AE\u91D1\u984D:",
        "commissionRate": "\u4F63\u91D1\u6BD4\u4F8B:"
      },
      "queue": {
        "title": "\u968A\u5217\u72C0\u614B",
        "metrics": {
          "pending": "\u5F85\u8655\u7406 {{count}}",
          "maxWait": "\u6700\u9577\u7B49\u5F85 {{time}}",
          "backlog": "\u7A4D\u58D3",
          "processes": "\u9032\u7A0B",
          "pendingLabel": "\u5F85\u8655\u7406",
          "wait": "\u7B49\u5F85"
        },
        "jobDetails": "\u4F5C\u696D\u8A73\u60C5",
        "workload": "\u968A\u5217\u8CA0\u8F09",
        "workloadCount": "\u5171 {{count}} \u500B\u968A\u5217",
        "status": {
          "description": "Horizon \u904B\u884C\u72C0\u614B\u8207\u5404\u968A\u5217\u7A4D\u58D3",
          "running": "\u904B\u884C\u72C0\u614B",
          "normal": "\u6B63\u5E38",
          "abnormal": "\u7570\u5E38",
          "waitTime": "\u7576\u524D\u7B49\u5F85\u6642\u9593\uFF1A{{seconds}} \u79D2",
          "pending": "\u7B49\u5F85\u4E2D",
          "processing": "\u8655\u7406\u4E2D",
          "completed": "\u5DF2\u5B8C\u6210",
          "failed": "\u5931\u6557",
          "cancelled": "\u5DF2\u53D6\u6D88"
        },
        "details": {
          "description": "\u968A\u5217\u8655\u7406\u8A73\u7D30\u4FE1\u606F",
          "recentJobs": "\u8FD1\u671F\u4EFB\u52D9\u6578",
          "statisticsPeriod": "\u7D71\u8A08\u6642\u9593\u7BC4\u570D\uFF1A{{hours}} \u5C0F\u6642",
          "jobsPerMinute": "\u6BCF\u5206\u9418\u8655\u7406\u91CF",
          "maxThroughput": "\u6700\u9AD8\u541E\u5410\u91CF\uFF1A{{value}}",
          "failedJobs7Days": "7\u65E5\u5831\u932F\u6578\u91CF",
          "retentionPeriod": "\u4FDD\u7559 {{hours}} \u5C0F\u6642",
          "longestRunningQueue": "\u6700\u9577\u904B\u884C\u968A\u5217",
          "activeProcesses": "\u6D3B\u8E8D\u9032\u7A0B",
          "id": "\u4F5C\u696DID",
          "type": "\u4F5C\u696D\u985E\u578B",
          "status": "\u72C0\u614B",
          "progress": "\u9032\u5EA6",
          "createdAt": "\u5275\u5EFA\u6642\u9593",
          "updatedAt": "\u66F4\u65B0\u6642\u9593",
          "error": "\u932F\u8AA4\u4FE1\u606F",
          "data": "\u4F5C\u696D\u6578\u64DA",
          "result": "\u7D50\u679C",
          "duration": "\u8017\u6642",
          "attempts": "\u91CD\u8A66\u6B21\u6578",
          "nextRetry": "\u4E0B\u6B21\u91CD\u8A66",
          "failedJobsDetailTitle": "\u5931\u6557\u4EFB\u52D9\u8A73\u60C5",
          "viewFailedJobs": "\u67E5\u770B\u5831\u932F\u8A73\u60C5",
          "jobDetailTitle": "\u4EFB\u52D9\u8A73\u7D30\u4FE1\u606F",
          "time": "\u6642\u9593",
          "queue": "\u968A\u5217",
          "name": "\u4EFB\u52D9\u540D\u7A31",
          "exception": "\u7570\u5E38\u4FE1\u606F",
          "noException": "\u66AB\u7121\u7570\u5E38\u4FE1\u606F",
          "noFailedJobs": "\u66AB\u7121\u5931\u6557\u4EFB\u52D9",
          "connection": "\u9023\u63A5\u985E\u578B",
          "payload": "\u4EFB\u52D9\u6578\u64DA",
          "viewDetail": "\u67E5\u770B\u8A73\u60C5",
          "action": "\u64CD\u4F5C"
        },
        "actions": {
          "retry": "\u91CD\u8A66",
          "cancel": "\u53D6\u6D88",
          "delete": "\u522A\u9664",
          "viewDetails": "\u67E5\u770B\u8A73\u60C5"
        },
        "empty": "\u968A\u5217\u4E2D\u66AB\u7121\u4F5C\u696D",
        "loading": "\u6B63\u5728\u52A0\u8F09\u968A\u5217\u72C0\u614B...",
        "error": "\u52A0\u8F09\u968A\u5217\u72C0\u614B\u5931\u6557"
      },
      "common": {
        "refresh": "\u5237\u65B0",
        "close": "\u95DC\u9589",
        "pagination": "\u7B2C {{current}}/{{total}} \u9801\uFF0C\u5171 {{count}} \u689D"
      },
      "search": {
        "loading": "\u641C\u7D22\u4E2D...",
        "noResults": "\u672A\u627E\u5230\u7D50\u679C",
        "placeholder": "\u641C\u7D22\u83DC\u55AE\u548C\u529F\u80FD...",
        "title": "\u83DC\u55AE\u5C0E\u822A"
      },
      "traffic": {
        "domain": "\u57DF\u540D",
        "monthlyTraffic": "\u6708\u6D41\u91CF",
        "rank": "\u6392\u540D",
        "title": "\u6D41\u91CF\u6392\u884C",
        "todayTraffic": "\u4ECA\u65E5\u6D41\u91CF"
      }
    },
    "order": {
      "title": "\u8A02\u55AE\u7BA1\u7406",
      "description": "\u5728\u9019\u88E1\u53EF\u4EE5\u67E5\u770B\u7528\u6236\u8A02\u55AE\uFF0C\u5305\u62EC\u5206\u914D\u3001\u67E5\u770B\u3001\u522A\u9664\u7B49\u64CD\u4F5C\u3002",
      "table": {
        "columns": {
          "tradeNo": "\u8A02\u55AE\u865F",
          "select": "\u9078\u64C7",
          "type": "\u985E\u578B",
          "user": "\u7528\u6236",
          "plan": "\u8A02\u95B1\u8A08\u5283",
          "period": "\u9031\u671F",
          "amount": "\u652F\u4ED8\u91D1\u984D",
          "status": "\u8A02\u55AE\u72C0\u614B",
          "commission": "\u4F63\u91D1",
          "commissionStatus": "\u4F63\u91D1\u72C0\u614B",
          "createdAt": "\u5275\u5EFA\u6642\u9593",
          "actions": "\u64CD\u4F5C"
        }
      },
      "type": {
        "NEW": "\u65B0\u8CFC",
        "RENEWAL": "\u7E8C\u8CBB",
        "UPGRADE": "\u5347\u7D1A",
        "RESET_FLOW": "\u6D41\u91CF\u91CD\u7F6E",
        "DEPOSIT": "\u9918\u984D\u5145\u503C"
      },
      "period": {
        "hour_price": "\u6642\u4ED8",
        "day_price": "\u65E5\u4ED8",
        "month_price": "\u6708\u4ED8",
        "quarter_price": "\u5B63\u4ED8",
        "half_year_price": "\u534A\u5E74\u4ED8",
        "year_price": "\u5E74\u4ED8",
        "two_year_price": "\u5169\u5E74\u4ED8",
        "three_year_price": "\u4E09\u5E74\u4ED8",
        "onetime_price": "\u4E00\u6B21\u6027",
        "reset_price": "\u6D41\u91CF\u91CD\u7F6E\u5305",
        "deposit": "\u9918\u984D\u5145\u503C",
        "hourly": "\u5C0F\u6642",
        "daily": "\u5929",
        "monthly": "\u6708",
        "quarterly": "\u5B63\u5EA6",
        "half_yearly": "\u534A\u5E74",
        "yearly": "\u5E74",
        "two_yearly": "\u5169\u5E74",
        "three_yearly": "\u4E09\u5E74",
        "onetime": "\u4E00\u6B21\u6027",
        "reset_traffic": "\u91CD\u7F6E\u6D41\u91CF"
      },
      "status": {
        "PENDING": "\u5F85\u652F\u4ED8",
        "PROCESSING": "\u958B\u901A\u4E2D",
        "CANCELLED": "\u5DF2\u53D6\u6D88",
        "COMPLETED": "\u5DF2\u5B8C\u6210",
        "DISCOUNTED": "\u5DF2\u6298\u62B5",
        "tooltip": "\u6A19\u8A18\u70BA[\u5DF2\u652F\u4ED8]\u5F8C\u5C07\u6703\u7531\u7CFB\u7D71\u9032\u884C\u958B\u901A\u5F8C\u4E26\u5B8C\u6210"
      },
      "commission": {
        "PENDING": "\u5F85\u78BA\u8A8D",
        "PROCESSING": "\u767C\u653E\u4E2D",
        "VALID": "\u6709\u6548",
        "INVALID": "\u7121\u6548"
      },
      "filter": {
        "allTypes": "\u5168\u90E8\u985E\u578B",
        "allPeriods": "\u5168\u90E8\u9031\u671F",
        "allStatuses": "\u5168\u90E8\u72C0\u614B",
        "allCommissions": "\u5168\u90E8\u4F63\u91D1\u72C0\u614B",
        "userId": "\u7528\u6236 ID",
        "clear": "\u6E05\u9664",
        "clearAll": "\u6E05\u9664\u7BE9\u9078"
      },
      "actions": {
        "view": "\u67E5\u770B\u8A73\u60C5",
        "markAsPaid": "\u6A19\u8A18\u70BA\u5DF2\u652F\u4ED8",
        "cancel": "\u53D6\u6D88\u8A02\u55AE",
        "issue": "\u767C\u653E\u4F63\u91D1",
        "invalid": "\u7121\u6548\u4F63\u91D1",
        "openMenu": "\u6253\u958B\u83DC\u55AE",
        "reset": "\u91CD\u7F6E",
        "copyTradeNo": "\u8907\u88FD\u8A02\u55AE\u865F",
        "batchConfirmCommission": "\u6279\u91CF\u78BA\u8A8D\u4F63\u91D1 ({{count}})",
        "batchConfirmCommissionConfirmTitle": "\u78BA\u8A8D\u6279\u91CF\u767C\u653E\u4F63\u91D1",
        "batchConfirmCommissionConfirmButton": "\u78BA\u8A8D\u767C\u653E"
      },
      "search": {
        "placeholder": "\u641C\u7D22\u8A02\u55AE\u865F..."
      },
      "dialog": {
        "title": "\u8A02\u55AE\u4FE1\u606F",
        "basicInfo": "\u57FA\u672C\u4FE1\u606F",
        "amountInfo": "\u91D1\u984D\u4FE1\u606F",
        "timeInfo": "\u6642\u9593\u4FE1\u606F",
        "commissionInfo": "\u4F63\u91D1\u4FE1\u606F",
        "commissionStatusActive": "\u6709\u6548",
        "addOrder": "\u6DFB\u52A0\u8A02\u55AE",
        "assignOrder": "\u8A02\u55AE\u5206\u914D",
        "fields": {
          "userEmail": "\u7528\u6236\u90F5\u7BB1",
          "userPhone": "\u7528\u6236\u624B\u6A5F\u865F",
          "orderPeriod": "\u8A02\u55AE\u9031\u671F",
          "subscriptionPlan": "\u8A02\u95B1\u8A08\u5283",
          "callbackNo": "\u56DE\u8ABF\u55AE\u865F",
          "paymentAmount": "\u652F\u4ED8\u91D1\u984D",
          "balancePayment": "\u9918\u984D\u652F\u4ED8",
          "discountAmount": "\u512A\u60E0\u91D1\u984D",
          "refundAmount": "\u9000\u56DE\u91D1\u984D",
          "deductionAmount": "\u6298\u62B5\u91D1\u984D",
          "createdAt": "\u5275\u5EFA\u6642\u9593",
          "updatedAt": "\u66F4\u65B0\u6642\u9593",
          "commissionStatus": "\u4F63\u91D1\u72C0\u614B",
          "commissionAmount": "\u4F63\u91D1\u91D1\u984D",
          "actualCommissionAmount": "\u5BE6\u969B\u4F63\u91D1",
          "inviteUser": "\u9080\u8ACB\u4EBA",
          "inviteUserId": "\u9080\u8ACB\u4EBAID"
        },
        "placeholders": {
          "email": "\u8ACB\u8F38\u5165\u7528\u6236\u90F5\u7BB1",
          "plan": "\u8ACB\u9078\u64C7\u8A02\u95B1\u8A08\u5283",
          "period": "\u8ACB\u9078\u64C7\u8CFC\u8CB7\u6642\u9577",
          "amount": "\u8ACB\u8F38\u5165\u9700\u8981\u652F\u4ED8\u7684\u91D1\u984D"
        },
        "actions": {
          "cancel": "\u53D6\u6D88",
          "confirm": "\u78BA\u5B9A"
        },
        "messages": {
          "addSuccess": "\u6DFB\u52A0\u6210\u529F",
          "addOrder": "\u6DFB\u52A0\u8A02\u55AE"
        }
      },
      "messages": {
        "addSuccess": "\u6DFB\u52A0\u6210\u529F",
        "markPaidSuccess": "\u5DF2\u6A19\u8A18\u70BA\u5DF2\u652F\u4ED8",
        "cancelSuccess": "\u8A02\u55AE\u5DF2\u53D6\u6D88",
        "cancelConfirm": "\u78BA\u8A8D\u53D6\u6D88\u8A72\u8A02\u55AE\uFF1F",
        "commissionIssueSuccess": "\u5DF2\u767C\u653E\u4F63\u91D1",
        "commissionInvalidSuccess": "\u5DF2\u6A19\u8A18\u70BA\u7121\u6548\u4F63\u91D1",
        "commissionInvalidConfirm": "\u78BA\u8A8D\u5C07\u8A72\u4F63\u91D1\u6A19\u8A18\u70BA\u7121\u6548\uFF1F",
        "bulkCommissionConfirmConfirm": "\u78BA\u5B9A\u8981\u78BA\u8A8D\u9078\u4E2D\u7684 {{count}} \u500B\u8A02\u55AE\u4F63\u91D1\u55CE\uFF1F\u78BA\u8A8D\u5F8C\u5C07\u9032\u5165\u767C\u653E\u6D41\u7A0B\u3002",
        "bulkCommissionConfirmSuccess": "\u5DF2\u78BA\u8A8D {{count}} \u7B46\u4F63\u91D1"
      }
    }
  };
  window.FBOARD_TRANSLATIONS = window.FBOARD_TRANSLATIONS ?? {};
  window.FBOARD_TRANSLATIONS["zh-TW"] = translations;
})();
