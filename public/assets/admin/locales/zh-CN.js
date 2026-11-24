(() => {
  const translations = {
    "giftCard": {
      "title": "\u793C\u54C1\u5361\u7BA1\u7406",
      "description": "\u5728\u8FD9\u91CC\u53EF\u4EE5\u7BA1\u7406\u793C\u54C1\u5361\u6A21\u677F\u3001\u5151\u6362\u7801\u548C\u4F7F\u7528\u8BB0\u5F55\u7B49\u529F\u80FD\u3002",
      "tabs": {
        "templates": "\u6A21\u677F\u7BA1\u7406",
        "codes": "\u5151\u6362\u7801\u7BA1\u7406",
        "usages": "\u4F7F\u7528\u8BB0\u5F55",
        "statistics": "\u7EDF\u8BA1\u6570\u636E"
      },
      "template": {
        "title": "\u6A21\u677F\u7BA1\u7406",
        "description": "\u7BA1\u7406\u793C\u54C1\u5361\u6A21\u677F\uFF0C\u5305\u62EC\u521B\u5EFA\u3001\u7F16\u8F91\u548C\u5220\u9664\u6A21\u677F\u3002",
        "table": {
          "title": "\u6A21\u677F\u5217\u8868",
          "columns": {
            "id": "ID",
            "name": "\u540D\u79F0",
            "type": "\u7C7B\u578B",
            "status": "\u72B6\u6001",
            "sort": "\u6392\u5E8F",
            "rewards": "\u5956\u52B1\u5185\u5BB9",
            "created_at": "\u521B\u5EFA\u65F6\u95F4",
            "actions": "\u64CD\u4F5C",
            "no_rewards": "\u65E0\u5956\u52B1"
          }
        },
        "form": {
          "add": "\u6DFB\u52A0\u6A21\u677F",
          "edit": "\u7F16\u8F91\u6A21\u677F",
          "name": {
            "label": "\u6A21\u677F\u540D\u79F0",
            "placeholder": "\u8BF7\u8F93\u5165\u6A21\u677F\u540D\u79F0",
            "required": "\u8BF7\u8F93\u5165\u6A21\u677F\u540D\u79F0"
          },
          "sort": {
            "label": "\u6392\u5E8F",
            "placeholder": "\u6570\u5B57\u8D8A\u5C0F\u8D8A\u9760\u524D"
          },
          "type": {
            "label": "\u7C7B\u578B",
            "placeholder": "\u8BF7\u9009\u62E9\u793C\u54C1\u5361\u7C7B\u578B"
          },
          "description": {
            "label": "\u63CF\u8FF0",
            "placeholder": "\u8BF7\u8F93\u5165\u793C\u54C1\u5361\u63CF\u8FF0"
          },
          "status": {
            "label": "\u72B6\u6001",
            "description": "\u7981\u7528\u540E\uFF0C\u6B64\u6A21\u677F\u5C06\u65E0\u6CD5\u751F\u6210\u6216\u5151\u6362\u65B0\u7684\u793C\u54C1\u5361\u3002"
          },
          "display": {
            "title": "\u663E\u793A\u6548\u679C",
            "theme_color": {
              "label": "\u4E3B\u9898\u989C\u8272"
            },
            "icon": {
              "label": "\u56FE\u6807",
              "placeholder": "\u8BF7\u8F93\u5165\u56FE\u6807\u7684URL"
            },
            "background_image": {
              "label": "\u80CC\u666F\u56FE\u7247",
              "placeholder": "\u8BF7\u8F93\u5165\u80CC\u666F\u56FE\u7247\u7684URL"
            }
          },
          "conditions": {
            "title": "\u4F7F\u7528\u6761\u4EF6",
            "new_user_max_days": {
              "label": "\u65B0\u7528\u6237\u6CE8\u518C\u5929\u6570\u9650\u5236",
              "placeholder": "\u4F8B\u5982: 7 (\u4EC5\u9650\u6CE8\u518C7\u5929\u5185\u7684\u7528\u6237)"
            },
            "new_user_only": {
              "label": "\u4EC5\u9650\u65B0\u7528\u6237",
              "hint": "\u9700\u6CE8\u518C\u5929\u6570 \u2264 {{days}}"
            },
            "paid_user_only": {
              "label": "\u4EC5\u9650\u4ED8\u8D39\u7528\u6237"
            },
            "require_invite": {
              "label": "\u9700\u8981\u9080\u8BF7\u5173\u7CFB"
            },
            "allowed_plans": {
              "label": "\u5141\u8BB8\u7684\u5957\u9910",
              "placeholder": "\u9009\u62E9\u5141\u8BB8\u5151\u6362\u7684\u5957\u9910 (\u7559\u7A7A\u5219\u4E0D\u9650\u5236)"
            },
            "disallowed_plans": {
              "label": "\u7981\u6B62\u7684\u5957\u9910",
              "placeholder": "\u9009\u62E9\u7981\u6B62\u5151\u6362\u7684\u5957\u9910 (\u7559\u7A7A\u5219\u4E0D\u9650\u5236)"
            }
          },
          "limits": {
            "title": "\u4F7F\u7528\u9650\u5236",
            "max_use_per_user": {
              "label": "\u5355\u7528\u6237\u6700\u5927\u4F7F\u7528\u6B21\u6570",
              "placeholder": "\u7559\u7A7A\u5219\u4E0D\u9650\u5236"
            },
            "cooldown_hours": {
              "label": "\u540C\u7C7B\u5361\u51B7\u5374\u65F6\u95F4(\u5C0F\u65F6)",
              "placeholder": "\u7559\u7A7A\u5219\u4E0D\u9650\u5236"
            }
          },
          "rewards": {
            "title": "\u5956\u52B1\u5185\u5BB9",
            "invite_reward_rate": {
              "label": "\u9080\u8BF7\u4EBA\u5956\u52B1\u6BD4\u4F8B",
              "placeholder": "\u4F8B\u5982: 0.2 (\u4EE3\u886820%)",
              "description": "\u4F7F\u7528\u8005\u6709\u9080\u8BF7\u4EBA\u65F6\uFF0C\u7ED9\u9080\u8BF7\u4EBA\u7684\u5956\u52B1 = \u4F59\u989D\u5956\u52B1 * \u6B64\u6BD4\u4F8B"
            },
            "balance": {
              "label": "\u5956\u52B1\u4F59\u989D (\u5143)",
              "short_label": "\u4F59\u989D",
              "placeholder": "\u8BF7\u8F93\u5165\u5956\u52B1\u7684\u91D1\u989D(\u5143)"
            },
            "transfer_enable": {
              "label": "\u5956\u52B1\u6D41\u91CF (GB)",
              "short_label": "\u6D41\u91CF",
              "placeholder": "\u8BF7\u8F93\u5165\u5956\u52B1\u7684\u6D41\u91CF(GB)"
            },
            "expire_days": {
              "label": "\u5EF6\u957F\u6709\u6548\u671F (\u5929)",
              "short_label": "\u6709\u6548\u671F",
              "placeholder": "\u8BF7\u8F93\u5165\u5EF6\u957F\u7684\u5929\u6570"
            },
            "transfer": {
              "label": "\u5956\u52B1\u6D41\u91CF (\u5B57\u8282)",
              "placeholder": "\u8BF7\u8F93\u5165\u5956\u52B1\u7684\u6D41\u91CF(\u5B57\u8282)"
            },
            "days": {
              "label": "\u5EF6\u957F\u6709\u6548\u671F (\u5929)",
              "placeholder": "\u8BF7\u8F93\u5165\u5EF6\u957F\u7684\u5929\u6570"
            },
            "device_limit": {
              "label": "\u589E\u52A0\u8BBE\u5907\u6570",
              "short_label": "\u8BBE\u5907\u6570",
              "placeholder": "\u8BF7\u8F93\u5165\u589E\u52A0\u7684\u8BBE\u5907\u6570\u91CF"
            },
            "reset_package": {
              "label": "\u91CD\u7F6E\u5F53\u6708\u6D41\u91CF",
              "description": "\u5F00\u542F\u540E\uFF0C\u5151\u6362\u65F6\u4F1A\u5C06\u7528\u6237\u5F53\u524D\u5957\u9910\u7684\u5DF2\u7528\u6D41\u91CF\u6E05\u96F6\u3002"
            },
            "reset_count": {
              "description": "\u8BE5\u7C7B\u578B\u5361\u5C06\u91CD\u7F6E\u7528\u6237\u5F53\u6708\u7684\u6D41\u91CF\u4F7F\u7528\u3002"
            },
            "task_card": {
              "description": "\u4EFB\u52A1\u793C\u54C1\u5361\u7684\u5177\u4F53\u5956\u52B1\u5C06\u5728\u4EFB\u52A1\u7CFB\u7EDF\u4E2D\u914D\u7F6E\u3002"
            },
            "plan_id": {
              "label": "\u6307\u5B9A\u5957\u9910",
              "short_label": "\u5957\u9910",
              "placeholder": "\u8BF7\u9009\u62E9\u4E00\u4E2A\u5957\u9910"
            },
            "plan_validity_days": {
              "label": "\u5957\u9910\u6709\u6548\u671F (\u5929)",
              "short_label": "\u5957\u9910\u6709\u6548\u671F",
              "placeholder": "\u7559\u7A7A\u5219\u4F7F\u7528\u5957\u9910\u9ED8\u8BA4\u6709\u6548\u671F"
            },
            "random_rewards": {
              "label": "\u968F\u673A\u5956\u52B1\u6C60",
              "add": "\u6DFB\u52A0\u968F\u673A\u5956\u52B1\u9879",
              "weight": "\u6743\u91CD"
            }
          },
          "special_config": {
            "title": "\u7279\u6B8A\u914D\u7F6E",
            "start_time": {
              "label": "\u6D3B\u52A8\u5F00\u59CB\u65F6\u95F4",
              "placeholder": "\u8BF7\u9009\u62E9\u5F00\u59CB\u65E5\u671F"
            },
            "end_time": {
              "label": "\u6D3B\u52A8\u7ED3\u675F\u65F6\u95F4",
              "placeholder": "\u8BF7\u9009\u62E9\u7ED3\u675F\u65E5\u671F"
            },
            "festival_bonus": {
              "label": "\u8282\u65E5\u5956\u52B1\u4E58\u6570",
              "placeholder": "\u4F8B\u5982: 1.5 (\u4EE3\u88681.5\u500D)"
            }
          },
          "submit": {
            "saving": "\u4FDD\u5B58\u4E2D...",
            "save": "\u4FDD\u5B58"
          }
        },
        "actions": {
          "edit": "\u7F16\u8F91",
          "delete": "\u5220\u9664",
          "deleteConfirm": {
            "title": "\u786E\u8BA4\u5220\u9664",
            "description": "\u6B64\u64CD\u4F5C\u5C06\u6C38\u4E45\u5220\u9664\u8BE5\u6A21\u677F\uFF0C\u786E\u5B9A\u8981\u7EE7\u7EED\u5417\uFF1F",
            "confirmText": "\u5220\u9664"
          }
        }
      },
      "code": {
        "title": "\u5151\u6362\u7801\u7BA1\u7406",
        "form": {
          "generate": "\u751F\u6210\u5151\u6362\u7801",
          "template_id": {
            "label": "\u9009\u62E9\u6A21\u677F",
            "placeholder": "\u8BF7\u9009\u62E9\u4E00\u4E2A\u6A21\u677F\u6765\u751F\u6210\u5151\u6362\u7801"
          },
          "count": {
            "label": "\u751F\u6210\u6570\u91CF"
          },
          "prefix": {
            "label": "\u81EA\u5B9A\u4E49\u524D\u7F00 (\u53EF\u9009)"
          },
          "expires_hours": {
            "label": "\u6709\u6548\u671F (\u5C0F\u65F6)"
          },
          "max_usage": {
            "label": "\u6700\u5927\u4F7F\u7528\u6B21\u6570"
          },
          "download_csv": "\u5BFC\u51FACSV",
          "submit": {
            "generating": "\u751F\u6210\u4E2D...",
            "generate": "\u7ACB\u5373\u751F\u6210"
          }
        },
        "description": "\u7BA1\u7406\u793C\u54C1\u5361\u5151\u6362\u7801\uFF0C\u5305\u62EC\u751F\u6210\u3001\u67E5\u770B\u548C\u5BFC\u51FA\u5151\u6362\u7801\u3002",
        "generate": {
          "title": "\u751F\u6210\u5151\u6362\u7801",
          "template": "\u9009\u62E9\u6A21\u677F",
          "count": "\u751F\u6210\u6570\u91CF",
          "prefix": "\u81EA\u5B9A\u4E49\u524D\u7F00",
          "expires_hours": "\u6709\u6548\u671F (\u5C0F\u65F6)",
          "max_usage": "\u6700\u5927\u4F7F\u7528\u6B21\u6570",
          "submit": "\u751F\u6210"
        },
        "table": {
          "title": "\u5151\u6362\u7801\u5217\u8868",
          "columns": {
            "id": "ID",
            "code": "\u5151\u6362\u7801",
            "template_name": "\u6A21\u677F\u540D\u79F0",
            "status": "\u72B6\u6001",
            "expires_at": "\u8FC7\u671F\u65F6\u95F4",
            "usage_count": "\u5DF2\u7528\u6B21\u6570",
            "max_usage": "\u53EF\u7528\u6B21\u6570",
            "created_at": "\u521B\u5EFA\u65F6\u95F4"
          }
        },
        "actions": {
          "enable": "\u542F\u7528",
          "disable": "\u7981\u7528",
          "export": "\u5BFC\u51FA",
          "exportConfirm": {
            "title": "\u786E\u8BA4\u5BFC\u51FA",
            "description": "\u5C06\u5BFC\u51FA\u9009\u5B9A\u6279\u6B21\u7684\u6240\u6709\u5151\u6362\u7801\u4E3A\u6587\u672C\u6587\u4EF6\u3002\u786E\u5B9A\u8981\u7EE7\u7EED\u5417\uFF1F",
            "confirmText": "\u5BFC\u51FA"
          }
        },
        "status": {
          "0": "\u672A\u4F7F\u7528",
          "1": "\u5DF2\u4F7F\u7528",
          "2": "\u5DF2\u8FC7\u671F",
          "3": "\u5DF2\u7981\u7528"
        },
        "edit": {
          "title": "\u7F16\u8F91\u793C\u54C1\u7801",
          "code": "\u793C\u54C1\u7801",
          "template": "\u6A21\u677F",
          "templatePlaceholder": "\u8BF7\u9009\u62E9\u6A21\u677F",
          "maxUsage": "\u6700\u5927\u4F7F\u7528\u6B21\u6570",
          "status": "\u72B6\u6001",
          "expiresAt": "\u8FC7\u671F\u65F6\u95F4"
        },
        "messages": {
          "enabled": "\u5DF2\u542F\u7528",
          "disabled": "\u5DF2\u7981\u7528",
          "exportSuccess": "\u5BFC\u51FA\u6210\u529F",
          "deleteConfirmTitle": "\u786E\u8BA4\u5220\u9664",
          "deleteConfirmDescription": "\u786E\u5B9A\u8981\u5220\u9664\u793C\u54C1\u7801 {{code}} \u5417\uFF1F\u6B64\u64CD\u4F5C\u4E0D\u53EF\u64A4\u9500\u3002",
          "deleteSuccess": "\u5220\u9664\u6210\u529F",
          "selectTemplate": "\u8BF7\u9009\u62E9\u6A21\u677F",
          "updateSuccess": "\u66F4\u65B0\u6210\u529F"
        }
      },
      "usage": {
        "title": "\u4F7F\u7528\u8BB0\u5F55",
        "description": "\u67E5\u770B\u793C\u54C1\u5361\u7684\u4F7F\u7528\u8BB0\u5F55\u548C\u8BE6\u7EC6\u4FE1\u606F\u3002",
        "table": {
          "columns": {
            "id": "ID",
            "code": "\u5151\u6362\u7801",
            "template_name": "\u6A21\u677F\u540D\u79F0",
            "user_email": "\u7528\u6237\u90AE\u7BB1",
            "rewards_given": "\u83B7\u5F97\u5956\u52B1",
            "invite_rewards": "\u9080\u8BF7\u5956\u52B1",
            "multiplier_applied": "\u500D\u6570\u52A0\u6210",
            "ip_address": "IP\u5730\u5740",
            "created_at": "\u4F7F\u7528\u65F6\u95F4",
            "actions": "\u64CD\u4F5C"
          }
        },
        "actions": {
          "view": "\u67E5\u770B\u8BE6\u60C5"
        }
      },
      "statistics": {
        "title": "\u7EDF\u8BA1\u6570\u636E",
        "description": "\u67E5\u770B\u793C\u54C1\u5361\u7684\u7EDF\u8BA1\u6570\u636E\u548C\u4F7F\u7528\u60C5\u51B5\u5206\u6790\u3002",
        "total": {
          "title": "\u603B\u4F53\u7EDF\u8BA1",
          "templates_count": "\u6A21\u677F\u603B\u6570",
          "active_templates_count": "\u6D3B\u8DC3\u6A21\u677F\u6570",
          "codes_count": "\u5151\u6362\u7801\u603B\u6570",
          "used_codes_count": "\u5DF2\u4F7F\u7528\u5151\u6362\u7801",
          "usages_count": "\u4F7F\u7528\u8BB0\u5F55\u6570"
        },
        "daily": {
          "title": "\u6BCF\u65E5\u4F7F\u7528\u91CF",
          "chart": "\u4F7F\u7528\u91CF\u8D8B\u52BF\u56FE"
        },
        "type": {
          "title": "\u7C7B\u578B\u7EDF\u8BA1",
          "chart": "\u7C7B\u578B\u5206\u5E03\u56FE"
        },
        "dateRange": {
          "label": "\u65E5\u671F\u8303\u56F4",
          "start": "\u5F00\u59CB\u65E5\u671F",
          "end": "\u7ED3\u675F\u65E5\u671F"
        }
      },
      "types": {
        "1": "\u901A\u7528\u793C\u54C1\u5361",
        "2": "\u5957\u9910\u793C\u54C1\u5361",
        "3": "\u76F2\u76D2\u793C\u54C1\u5361",
        "4": "\u4EFB\u52A1\u793C\u54C1\u5361"
      },
      "common": {
        "search": "\u641C\u7D22\u793C\u54C1\u5361...",
        "reset": "\u91CD\u7F6E",
        "filter": "\u7B5B\u9009",
        "export": "\u5BFC\u51FA",
        "refresh": "\u5237\u65B0",
        "back": "\u8FD4\u56DE",
        "close": "\u5173\u95ED",
        "confirm": "\u786E\u8BA4",
        "cancel": "\u53D6\u6D88",
        "enabled": "\u5DF2\u542F\u7528",
        "disabled": "\u5DF2\u7981\u7528",
        "loading": "\u52A0\u8F7D\u4E2D...",
        "noData": "\u6682\u65E0\u6570\u636E",
        "success": "\u64CD\u4F5C\u6210\u529F",
        "error": "\u64CD\u4F5C\u5931\u8D25"
      },
      "messages": {
        "formInvalid": "\u8BF7\u68C0\u67E5\u8868\u5355\u8F93\u5165\u662F\u5426\u6B63\u786E",
        "templateCreated": "\u6A21\u677F\u521B\u5EFA\u6210\u529F",
        "templateUpdated": "\u6A21\u677F\u66F4\u65B0\u6210\u529F",
        "templateDeleted": "\u6A21\u677F\u5220\u9664\u6210\u529F",
        "codeGenerated": "\u5151\u6362\u7801\u751F\u6210\u6210\u529F",
        "generateCodeFailed": "\u5151\u6362\u7801\u751F\u6210\u5931\u8D25",
        "codeStatusUpdated": "\u5151\u6362\u7801\u72B6\u6001\u66F4\u65B0\u6210\u529F",
        "updateCodeStatusFailed": "\u5151\u6362\u7801\u72B6\u6001\u66F4\u65B0\u5931\u8D25",
        "codesExported": "\u5151\u6362\u7801\u5BFC\u51FA\u6210\u529F",
        "createTemplateFailed": "\u521B\u5EFA\u6A21\u677F\u5931\u8D25",
        "updateTemplateFailed": "\u66F4\u65B0\u6A21\u677F\u5931\u8D25",
        "deleteTemplateFailed": "\u5220\u9664\u6A21\u677F\u5931\u8D25",
        "loadDataFailed": "\u52A0\u8F7D\u6570\u636E\u5931\u8D25",
        "codesGenerated": "\u5151\u6362\u7801\u751F\u6210\u6210\u529F"
      }
    },
    "user": {
      "manage": {
        "title": "\u7528\u6237\u7BA1\u7406",
        "description": "\u5728\u8FD9\u91CC\u53EF\u4EE5\u7BA1\u7406\u7528\u6237\uFF0C\u5305\u62EC\u589E\u52A0\u3001\u5220\u9664\u3001\u7F16\u8F91\u3001\u67E5\u8BE2\u7B49\u64CD\u4F5C\u3002"
      },
      "columns": {
        "is_admin": "\u7BA1\u7406\u5458",
        "is_staff": "\u5458\u5DE5",
        "id": "ID",
        "email": "\u90AE\u7BB1",
        "online_count": "\u5728\u7EBF\u8BBE\u5907",
        "status": "\u72B6\u6001",
        "subscription": "\u8BA2\u9605",
        "group": "\u6743\u9650\u7EC4",
        "used_traffic": "\u5DF2\u7528\u6D41\u91CF",
        "total_traffic": "\u603B\u6D41\u91CF",
        "expire_time": "\u5230\u671F\u65F6\u95F4",
        "balance": "\u4F59\u989D",
        "commission": "\u4F63\u91D1",
        "register_time": "\u6CE8\u518C\u65F6\u95F4",
        "invitee_email": "\u9080\u8BF7\u7528\u6237",
        "actions": "\u64CD\u4F5C",
        "next_reset_at": "\u4E0B\u6B21\u91CD\u7F6E\u65F6\u95F4",
        "device_limit": {
          "unlimited": "\u65E0\u8BBE\u5907\u6570\u9650\u5236",
          "limited": "\u6700\u591A\u53EF\u540C\u65F6\u5728\u7EBF {{count}} \u53F0\u8BBE\u5907"
        },
        "status_text": {
          "normal": "\u6B63\u5E38",
          "banned": "\u5DF2\u5C01\u7981"
        },
        "online_status": {
          "online": "\u5F53\u524D\u5728\u7EBF",
          "never": "\u4ECE\u672A\u5728\u7EBF",
          "last_online": "\u6700\u540E\u5728\u7EBF\u65F6\u95F4: {{time}}",
          "offline_duration": {
            "days": "\u79BB\u7EBF\u65F6\u957F: {{count}}\u5929",
            "hours": "\u79BB\u7EBF\u65F6\u957F: {{count}}\u5C0F\u65F6",
            "minutes": "\u79BB\u7EBF\u65F6\u957F: {{count}}\u5206\u949F",
            "seconds": "\u79BB\u7EBF\u65F6\u957F: {{count}}\u79D2"
          }
        },
        "expire_status": {
          "permanent": "\u957F\u671F\u6709\u6548",
          "expired": "\u5DF2\u8FC7\u671F {{days}} \u5929",
          "remaining": "\u5269\u4F59 {{days}} \u5929"
        },
        "copy_email": "\u590D\u5236\u90AE\u7BB1",
        "actions_menu": {
          "edit": "\u7F16\u8F91",
          "view_details": "\u67E5\u770B\u8BE6\u60C5",
          "assign_order": "\u5206\u914D\u8BA2\u5355",
          "copy_url": "\u590D\u5236\u8BA2\u9605URL",
          "reset_secret": "\u91CD\u7F6EUUID\u53CA\u8BA2\u9605URL",
          "reset_traffic": "\u91CD\u7F6E\u6D41\u91CF",
          "orders": "TA\u7684\u8BA2\u5355",
          "invites": "TA\u7684\u9080\u8BF7",
          "traffic_records": "TA\u7684\u6D41\u91CF\u8BB0\u5F55",
          "login_history": "\u767B\u5F55\u5386\u53F2",
          "delete": "\u5220\u9664",
          "delete_confirm_title": "\u786E\u8BA4\u5220\u9664\u7528\u6237",
          "delete_confirm_description": "\u6B64\u64CD\u4F5C\u5C06\u6C38\u4E45\u5220\u9664\u7528\u6237 {{email}} \u53CA\u5176\u6240\u6709\u76F8\u5173\u6570\u636E\uFF0C\u5305\u62EC\u8BA2\u5355\u3001\u4F18\u60E0\u7801\u3001\u6D41\u91CF\u8BB0\u5F55\u3001\u5DE5\u5355\u8BB0\u5F55\u7B49\u4FE1\u606F\u3002\u5220\u9664\u540E\u65E0\u6CD5\u6062\u590D\uFF0C\u662F\u5426\u7EE7\u7EED\uFF1F"
        }
      },
      "filter": {
        "selected": "\u5DF2\u9009\u62E9 {{count}} \u9879",
        "clear_selection": "\u53D6\u6D88\u9009\u62E9",
        "no_results": "\u672A\u627E\u5230\u7ED3\u679C",
        "clear": "\u6E05\u9664\u7B5B\u9009",
        "search_placeholder": "\u641C\u7D22...",
        "email_search": "\u641C\u7D22\u7528\u6237\u90AE\u7BB1...",
        "advanced": "\u9AD8\u7EA7\u7B5B\u9009",
        "reset": "\u91CD\u7F6E\u7B5B\u9009",
        "sheet": {
          "title": "\u9AD8\u7EA7\u7B5B\u9009",
          "description": "\u6DFB\u52A0\u4E00\u4E2A\u6216\u591A\u4E2A\u7B5B\u9009\u6761\u4EF6\u6765\u7CBE\u786E\u67E5\u627E\u7528\u6237",
          "conditions": "\u7B5B\u9009\u6761\u4EF6",
          "add": "\u6DFB\u52A0\u6761\u4EF6",
          "condition": "\u6761\u4EF6 {{number}}",
          "field": "\u9009\u62E9\u5B57\u6BB5",
          "operator": "\u9009\u62E9\u64CD\u4F5C\u7B26",
          "value": "\u8F93\u5165\u503C",
          "value_number": "\u8F93\u5165\u6570\u503C({{unit}})",
          "reset": "\u91CD\u7F6E",
          "apply": "\u5E94\u7528\u7B5B\u9009"
        },
        "fields": {
          "email": "\u90AE\u7BB1",
          "phone": "\u624B\u673A\u53F7",
          "id": "\u7528\u6237ID",
          "plan_id": "\u8BA2\u9605",
          "transfer_enable": "\u6D41\u91CF",
          "total_used": "\u5DF2\u7528\u6D41\u91CF",
          "online_count": "\u5728\u7EBF\u8BBE\u5907",
          "expired_at": "\u5230\u671F\u65F6\u95F4",
          "uuid": "UUID",
          "token": "Token",
          "banned": "\u8D26\u53F7\u72B6\u6001",
          "remark": "\u5907\u6CE8",
          "inviter_email": "\u9080\u8BF7\u4EBA\u90AE\u7BB1",
          "invite_user_id": "\u9080\u8BF7\u4EBAID",
          "is_admin": "\u7BA1\u7406\u5458",
          "is_staff": "\u5458\u5DE5"
        },
        "operators": {
          "contains": "\u5305\u542B",
          "eq": "\u7B49\u4E8E",
          "gt": "\u5927\u4E8E",
          "lt": "\u5C0F\u4E8E"
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
        "button": "\u521B\u5EFA\u7528\u6237",
        "title": "\u521B\u5EFA\u7528\u6237",
        "description_single": "\u521B\u5EFA\u5355\u4E2A\u6307\u5B9A\u90AE\u7BB1\u8D26\u53F7",
        "description_batch": "\u6279\u91CF\u751F\u6210\u968F\u673A\u90AE\u7BB1\u8D26\u53F7",
        "form": {
          "email": "\u90AE\u7BB1",
          "email_prefix": "\u5E10\u53F7(\u6279\u91CF\u751F\u6210\u8BF7\u7559\u7A7A)",
          "email_prefix_placeholder": "\u7559\u7A7A\u5219\u6279\u91CF\u968F\u673A",
          "email_domain": "\u57DF",
          "password": "\u5BC6\u7801",
          "password_placeholder": "\u7559\u7A7A\u5219\u5BC6\u7801\u4E0E\u90AE\u4EF6\u76F8\u540C",
          "expire_time": "\u5230\u671F\u65F6\u95F4",
          "expire_time_placeholder": "\u8BF7\u9009\u62E9\u7528\u6237\u5230\u671F\u65E5\u671F\uFF0C\u7559\u7A7A\u4E3A\u957F\u671F\u6709\u6548",
          "permanent": "\u957F\u671F\u6709\u6548",
          "subscription": "\u8BA2\u9605\u8BA1\u5212",
          "plans": "\u5957\u9910\u5217\u8868",
          "add_plan": "\u6DFB\u52A0\u5957\u9910",
          "subscription_none": "\u65E0",
          "generate_count": "\u751F\u6210\u6570\u91CF",
          "generate_count_placeholder": "\u5982\u679C\u4E3A\u6279\u91CF\u751F\u4EA7\u8BF7\u8F93\u5165\u751F\u6210\u6570\u91CF",
          "cancel": "\u53D6\u6D88",
          "submit": "\u751F\u6210",
          "success": "\u751F\u6210\u6210\u529F",
          "download_csv": "\u5BFC\u51FA\u4E3A CSV \u6587\u4EF6",
          "generated_count": "\u5DF2\u751F\u6210 {{count}} \u4E2A\u8D26\u53F7",
          "single_success_hint": "\u8D26\u53F7\u5DF2\u521B\u5EFA\uFF0C\u53EF\u5728\u7528\u6237\u5217\u8868\u4E2D\u67E5\u770B\u4E0E\u7F16\u8F91\u3002",
          "copy_all": "\u590D\u5236\u5168\u90E8",
          "copy_email": "\u590D\u5236\u90AE\u7BB1",
          "copy_subscribe": "\u590D\u5236\u8BA2\u9605",
          "result_password": "\u5BC6\u7801",
          "result_expire": "\u5230\u671F"
        },
        "copy_line": {
          "email": "\u90AE\u7BB1: {{value}}",
          "password": "\u5BC6\u7801: {{value}}",
          "expire": "\u5230\u671F: {{value}}",
          "subscribe": "\u8BA2\u9605: {{value}}"
        },
        "csv": {
          "email": "\u90AE\u7BB1",
          "password": "\u5BC6\u7801",
          "expire_time": "\u5230\u671F\u65F6\u95F4",
          "uuid": "UUID",
          "created_at": "\u521B\u5EFA\u65F6\u95F4",
          "subscribe_url": "\u8BA2\u9605\u5730\u5740"
        }
      },
      "edit": {
        "button": "\u7F16\u8F91\u7528\u6237\u4FE1\u606F",
        "title": "\u7528\u6237\u7BA1\u7406",
        "form": {
          "email": "\u90AE\u7BB1",
          "phone_country": "\u56FD\u5BB6/\u5730\u533A",
          "phone": "\u624B\u673A\u53F7",
          "phone_placeholder": "\u8BF7\u8F93\u5165\u624B\u673A\u53F7",
          "email_placeholder": "\u8BF7\u8F93\u5165\u90AE\u7BB1",
          "inviter_email": "\u9080\u8BF7\u4EBA\u90AE\u7BB1",
          "inviter_email_placeholder": "\u8BF7\u8F93\u5165\u90AE\u7BB1",
          "invite_user_id": "\u9080\u8BF7\u4EBAID",
          "invite_user_id_placeholder": "\u8BF7\u8F93\u5165\u9080\u8BF7\u4EBA\u7528\u6237ID\uFF0C\u7559\u7A7A\u5219\u6E05\u7A7A",
          "password": "\u5BC6\u7801",
          "password_placeholder": "\u5982\u9700\u4FEE\u6539\u5BC6\u7801\u8BF7\u8F93\u5165",
          "balance": "\u4F59\u989D",
          "balance_placeholder": "\u8BF7\u8F93\u5165\u4F59\u989D",
          "commission_balance": "\u4F63\u91D1\u4F59\u989D",
          "commission_balance_placeholder": "\u8BF7\u8F93\u5165\u4F63\u91D1\u4F59\u989D",
          "upload": "\u5DF2\u7528\u4E0A\u884C",
          "upload_placeholder": "\u5DF2\u7528\u4E0A\u884C",
          "download": "\u5DF2\u7528\u4E0B\u884C",
          "download_placeholder": "\u5DF2\u7528\u4E0B\u884C",
          "total_traffic": "\u6D41\u91CF",
          "total_traffic_placeholder": "\u8BF7\u8F93\u5165\u6D41\u91CF",
          "expire_time": "\u5230\u671F\u65F6\u95F4",
          "expire_time_placeholder": "\u8BF7\u9009\u62E9\u7528\u6237\u5230\u671F\u65E5\u671F\uFF0C\u7559\u7A7A\u4E3A\u957F\u671F\u6709\u6548",
          "expire_time_specific": "\u5177\u4F53\u65F6\u95F4",
          "expire_time_today": "\u8BBE\u4E3A\u5F53\u5929\u7ED3\u675F",
          "expire_time_permanent": "\u957F\u671F\u6709\u6548",
          "expire_time_1month": "\u4E00\u4E2A\u6708",
          "expire_time_3months": "\u4E09\u4E2A\u6708",
          "expire_time_confirm": "\u786E\u5B9A",
          "subscription": "\u8BA2\u9605\u8BA1\u5212",
          "subscription_none": "\u65E0",
          "account_status": "\u8D26\u6237\u72B6\u6001",
          "banned_hint": "\u7528\u6237\u65E0\u6CD5\u767B\u5F55\u3001\u65E0\u6CD5\u4F7F\u7528\u8BA2\u9605\u670D\u52A1",
          "normal_hint": "\u7528\u6237\u53EF\u4EE5\u6B63\u5E38\u4F7F\u7528\u6240\u6709\u670D\u52A1",
          "commission_type": "\u4F63\u91D1\u7C7B\u578B",
          "commission_type_system": "\u8DDF\u968F\u7CFB\u7EDF\u8BBE\u7F6E",
          "commission_type_cycle": "\u5FAA\u73AF\u8FD4\u5229",
          "commission_type_onetime": "\u9996\u6B21\u8FD4\u5229",
          "commission_rate": "\u63A8\u8350\u8FD4\u5229\u6BD4\u4F8B",
          "commission_rate_placeholder": "\u4E3A\u7A7A\u5219\u8DDF\u968F\u7AD9\u70B9\u8BBE\u7F6E\u8FD4\u5229\u6BD4\u4F8B",
          "discount": "\u4E13\u4EAB\u6298\u6263\u6BD4\u4F8B",
          "discount_placeholder": "\u4E3A\u7A7A\u5219\u4E0D\u4EAB\u53D7\u4E13\u4EAB\u6298\u6263",
          "speed_limit": "\u9650\u901F",
          "speed_limit_placeholder": "\u7559\u7A7A\u5219\u4E0D\u9650\u901F",
          "device_limit": "\u8BBE\u5907\u9650\u5236",
          "device_limit_placeholder": "\u7559\u7A7A\u5219\u4E0D\u9650\u5236",
          "is_admin": "\u662F\u5426\u7BA1\u7406\u5458",
          "is_staff": "\u662F\u5426\u5458\u5DE5",
          "remarks": "\u5907\u6CE8",
          "remarks_placeholder": "\u8BF7\u5728\u8FD9\u91CC\u8BB0\u5F55",
          "cancel": "\u53D6\u6D88",
          "submit": "\u63D0\u4EA4",
          "success": "\u4FEE\u6539\u6210\u529F"
        }
      },
      "actions": {
        "title": "\u64CD\u4F5C",
        "send_email": "\u53D1\u9001\u90AE\u4EF6",
        "export_csv": "\u5BFC\u51FA CSV",
        "traffic_reset_stats": "\u6D41\u91CF\u91CD\u7F6E\u7EDF\u8BA1",
        "batch_ban": "\u6279\u91CF\u5C01\u7981",
        "confirm_ban": {
          "title": "\u786E\u8BA4\u6279\u91CF\u5C01\u7981",
          "filtered_description": "\u6B64\u64CD\u4F5C\u5C06\u5C01\u7981\u6240\u6709\u7B26\u5408\u5F53\u524D\u7B5B\u9009\u6761\u4EF6\u7684\u7528\u6237\u3002\u6B64\u64CD\u4F5C\u65E0\u6CD5\u64A4\u9500\u3002",
          "all_description": "\u6B64\u64CD\u4F5C\u5C06\u5C01\u7981\u7CFB\u7EDF\u4E2D\u7684\u6240\u6709\u7528\u6237\u3002\u6B64\u64CD\u4F5C\u65E0\u6CD5\u64A4\u9500\u3002",
          "cancel": "\u53D6\u6D88",
          "confirm": "\u786E\u8BA4\u5C01\u7981",
          "banning": "\u5C01\u7981\u4E2D..."
        }
      },
      "traffic_reset": {
        "title": "\u6D41\u91CF\u91CD\u7F6E",
        "description": "\u4E3A\u7528\u6237 {{email}} \u91CD\u7F6E\u6D41\u91CF\u4F7F\u7528\u91CF",
        "tabs": {
          "reset": "\u91CD\u7F6E\u6D41\u91CF",
          "history": "\u91CD\u7F6E\u5386\u53F2"
        },
        "user_info": "\u7528\u6237\u4FE1\u606F",
        "warning": {
          "title": "\u91CD\u8981\u63D0\u9192",
          "irreversible": "\u6D41\u91CF\u91CD\u7F6E\u64CD\u4F5C\u4E0D\u53EF\u9006\uFF0C\u8BF7\u8C28\u614E\u64CD\u4F5C",
          "reset_to_zero": "\u91CD\u7F6E\u540E\u7528\u6237\u7684\u4E0A\u4F20\u548C\u4E0B\u8F7D\u6D41\u91CF\u5C06\u6E05\u96F6",
          "logged": "\u6240\u6709\u91CD\u7F6E\u64CD\u4F5C\u90FD\u4F1A\u88AB\u8BB0\u5F55\u5728\u7CFB\u7EDF\u65E5\u5FD7\u4E2D"
        },
        "reason": {
          "label": "\u91CD\u7F6E\u539F\u56E0",
          "placeholder": "\u8BF7\u8F93\u5165\u91CD\u7F6E\u6D41\u91CF\u7684\u539F\u56E0\uFF08\u53EF\u9009\uFF09",
          "optional": "\u6B64\u5B57\u6BB5\u4E3A\u53EF\u9009\u9879\uFF0C\u7528\u4E8E\u8BB0\u5F55\u91CD\u7F6E\u539F\u56E0"
        },
        "confirm_reset": "\u786E\u8BA4\u91CD\u7F6E",
        "resetting": "\u91CD\u7F6E\u4E2D...",
        "reset_success": "\u6D41\u91CF\u91CD\u7F6E\u6210\u529F",
        "reset_failed": "\u6D41\u91CF\u91CD\u7F6E\u5931\u8D25",
        "history": {
          "summary": "\u91CD\u7F6E\u6982\u89C8",
          "reset_count": "\u91CD\u7F6E\u6B21\u6570",
          "last_reset": "\u6700\u540E\u91CD\u7F6E",
          "next_reset": "\u4E0B\u6B21\u91CD\u7F6E",
          "never": "\u4ECE\u672A\u91CD\u7F6E",
          "no_schedule": "\u65E0\u5B9A\u65F6\u91CD\u7F6E",
          "records": "\u91CD\u7F6E\u8BB0\u5F55",
          "recent_records": "\u6700\u8FD110\u6B21\u91CD\u7F6E\u8BB0\u5F55",
          "no_records": "\u6682\u65E0\u91CD\u7F6E\u8BB0\u5F55",
          "reset_time": "\u91CD\u7F6E\u65F6\u95F4",
          "traffic_cleared": "\u6E05\u9664\u6D41\u91CF"
        },
        "stats": {
          "title": "\u6D41\u91CF\u91CD\u7F6E\u7EDF\u8BA1",
          "description": "\u67E5\u770B\u7CFB\u7EDF\u6D41\u91CF\u91CD\u7F6E\u7684\u7EDF\u8BA1\u4FE1\u606F",
          "time_range": "\u7EDF\u8BA1\u65F6\u95F4\u8303\u56F4",
          "total_resets": "\u603B\u91CD\u7F6E\u6B21\u6570",
          "auto_resets": "\u81EA\u52A8\u91CD\u7F6E",
          "manual_resets": "\u624B\u52A8\u91CD\u7F6E",
          "cron_resets": "\u5B9A\u65F6\u91CD\u7F6E",
          "in_period": "\u6700\u8FD1 {{days}} \u5929",
          "breakdown": "\u91CD\u7F6E\u7C7B\u578B\u5206\u5E03",
          "breakdown_description": "\u5404\u7C7B\u578B\u91CD\u7F6E\u64CD\u4F5C\u7684\u767E\u5206\u6BD4\u5206\u5E03",
          "auto_percentage": "\u81EA\u52A8\u91CD\u7F6E\u5360\u6BD4",
          "manual_percentage": "\u624B\u52A8\u91CD\u7F6E\u5360\u6BD4",
          "cron_percentage": "\u5B9A\u65F6\u91CD\u7F6E\u5360\u6BD4",
          "days_options": {
            "week": "\u6700\u8FD1\u4E00\u5468",
            "month": "\u6700\u8FD1\u4E00\u6708",
            "quarter": "\u6700\u8FD1\u4E09\u6708",
            "year": "\u6700\u8FD1\u4E00\u5E74"
          }
        }
      },
      "traffic_reset_logs": {
        "title": "\u6D41\u91CF\u91CD\u7F6E\u65E5\u5FD7",
        "description": "\u67E5\u770B\u7CFB\u7EDF\u4E2D\u6240\u6709\u6D41\u91CF\u91CD\u7F6E\u64CD\u4F5C\u7684\u8BE6\u7EC6\u8BB0\u5F55",
        "columns": {
          "id": "\u65E5\u5FD7ID",
          "user": "\u7528\u6237",
          "reset_type": "\u91CD\u7F6E\u7C7B\u578B",
          "trigger_source": "\u89E6\u53D1\u6E90",
          "cleared_traffic": "\u6E05\u9664\u6D41\u91CF",
          "cleared": "\u5DF2\u6E05\u9664",
          "upload": "\u4E0A\u4F20",
          "download": "\u4E0B\u8F7D",
          "reset_time": "\u91CD\u7F6E\u65F6\u95F4",
          "log_time": "\u8BB0\u5F55\u65F6\u95F4"
        },
        "filters": {
          "search_user": "\u641C\u7D22\u7528\u6237\u90AE\u7BB1...",
          "reset_type": "\u91CD\u7F6E\u7C7B\u578B",
          "trigger_source": "\u89E6\u53D1\u6E90",
          "all_types": "\u5168\u90E8\u7C7B\u578B",
          "all_sources": "\u5168\u90E8\u6765\u6E90",
          "start_date": "\u5F00\u59CB\u65E5\u671F",
          "end_date": "\u7ED3\u675F\u65E5\u671F",
          "apply_date": "\u5E94\u7528\u7B5B\u9009",
          "reset": "\u91CD\u7F6E\u7B5B\u9009",
          "filter_title": "\u7B5B\u9009\u6761\u4EF6",
          "filter_description": "\u8BBE\u7F6E\u7B5B\u9009\u6761\u4EF6\u6765\u67E5\u627E\u7279\u5B9A\u7684\u6D41\u91CF\u91CD\u7F6E\u8BB0\u5F55",
          "reset_types": {
            "monthly": "\u6309\u6708\u91CD\u7F6E",
            "first_day_month": "\u6BCF\u67081\u53F7\u91CD\u7F6E",
            "yearly": "\u6309\u5E74\u91CD\u7F6E",
            "first_day_year": "\u6BCF\u5E741\u67081\u65E5\u91CD\u7F6E",
            "manual": "\u624B\u52A8\u91CD\u7F6E"
          },
          "trigger_sources": {
            "auto": "\u81EA\u52A8\u89E6\u53D1",
            "manual": "\u624B\u52A8\u89E6\u53D1",
            "cron": "\u5B9A\u65F6\u4EFB\u52A1"
          }
        },
        "actions": {
          "export": "\u5BFC\u51FA\u65E5\u5FD7",
          "exporting": "\u5BFC\u51FA\u4E2D...",
          "export_success": "\u5BFC\u51FA\u6210\u529F",
          "export_failed": "\u5BFC\u51FA\u5931\u8D25"
        },
        "trigger_descriptions": {
          "manual": "\u7BA1\u7406\u5458\u624B\u52A8\u6267\u884C\u7684\u6D41\u91CF\u91CD\u7F6E",
          "cron": "\u7CFB\u7EDF\u5B9A\u65F6\u4EFB\u52A1\u81EA\u52A8\u6267\u884C",
          "auto": "\u7CFB\u7EDF\u6839\u636E\u6761\u4EF6\u81EA\u52A8\u89E6\u53D1",
          "other": "\u5176\u4ED6\u65B9\u5F0F\u89E6\u53D1"
        }
      },
      "login_history": {
        "title": "\u767B\u5F55\u5386\u53F2",
        "no_records": "\u6682\u65E0\u767B\u5F55\u8BB0\u5F55",
        "columns": {
          "time": "\u65F6\u95F4",
          "ip": "IP",
          "method": "\u65B9\u5F0F",
          "user_agent": "User-Agent"
        },
        "methods": {
          "password": "\u5BC6\u7801\u767B\u5F55",
          "register": "\u6CE8\u518C",
          "mail_link": "\u90AE\u4EF6\u94FE\u63A5"
        }
      },
      "messages": {
        "success": "\u6210\u529F",
        "error": "\u9519\u8BEF",
        "export": {
          "success": "\u5BFC\u51FA\u6210\u529F",
          "failed": "\u5BFC\u51FA\u5931\u8D25"
        },
        "batch_ban": {
          "success": "\u6279\u91CF\u5C01\u7981\u6210\u529F",
          "failed": "\u6279\u91CF\u5C01\u7981\u5931\u8D25"
        },
        "send_mail": {
          "success": "\u90AE\u4EF6\u53D1\u9001\u6210\u529F",
          "failed": "\u90AE\u4EF6\u53D1\u9001\u5931\u8D25",
          "required_fields": "\u8BF7\u586B\u5199\u6240\u6709\u5FC5\u586B\u5B57\u6BB5"
        },
        "reset_secret": {
          "success": "UUID & Token \u5DF2\u91CD\u7F6E"
        }
      },
      "send_mail": {
        "title": "\u53D1\u9001\u90AE\u4EF6",
        "description": "\u5411\u6240\u9009\u6216\u5DF2\u7B5B\u9009\u7684\u7528\u6237\u53D1\u9001\u90AE\u4EF6",
        "subject": "\u4E3B\u9898",
        "content": "\u5185\u5BB9",
        "sending": "\u53D1\u9001\u4E2D...",
        "send": "\u53D1\u9001"
      },
      "dialog": {
        "title": "\u7528\u6237\u8BE6\u60C5",
        "basicInfo": "\u57FA\u672C\u4FE1\u606F",
        "subscriptionInfo": "\u8BA2\u9605\u4FE1\u606F",
        "trafficInfo": "\u6D41\u91CF\u4FE1\u606F",
        "financialInfo": "\u8D22\u52A1\u4FE1\u606F",
        "activityInfo": "\u6D3B\u52A8\u4FE1\u606F",
        "inviteInfo": "\u9080\u8BF7\u4FE1\u606F",
        "timeInfo": "\u65F6\u95F4\u4FE1\u606F",
        "subscriptionUrl": "\u8BA2\u9605\u94FE\u63A5",
        "fields": {
          "userId": "\u7528\u6237ID",
          "email": "\u90AE\u7BB1",
          "phone": "\u624B\u673A\u53F7",
          "uuid": "UUID",
          "token": "Token",
          "remarks": "\u5907\u6CE8",
          "subscriptionPlan": "\u8BA2\u9605\u5957\u9910",
          "permissionGroup": "\u6743\u9650\u7EC4",
          "expiredAt": "\u5230\u671F\u65F6\u95F4",
          "deviceLimit": "\u8BBE\u5907\u9650\u5236",
          "speedLimit": "\u901F\u5EA6\u9650\u5236",
          "transferEnable": "\u603B\u6D41\u91CF",
          "uploadUsed": "\u4E0A\u4F20\u5DF2\u7528",
          "downloadUsed": "\u4E0B\u8F7D\u5DF2\u7528",
          "totalUsed": "\u603B\u5DF2\u7528",
          "lastResetAt": "\u4E0A\u6B21\u91CD\u7F6E",
          "nextResetAt": "\u4E0B\u6B21\u91CD\u7F6E",
          "resetCount": "\u91CD\u7F6E\u6B21\u6570",
          "balance": "\u4F59\u989D",
          "commissionBalance": "\u4F63\u91D1\u4F59\u989D",
          "commissionType": "\u4F63\u91D1\u7C7B\u578B",
          "commissionRate": "\u4F63\u91D1\u6BD4\u4F8B",
          "lastLoginAt": "\u6700\u540E\u767B\u5F55",
          "lastLoginIp": "\u6700\u540E\u767B\u5F55IP",
          "registerIp": "\u6CE8\u518CIP",
          "lastOnlineAt": "\u6700\u540E\u5728\u7EBF",
          "onlineCount": "\u5728\u7EBF\u8BBE\u5907",
          "inviteUser": "\u9080\u8BF7\u4EBA",
          "inviteUserId": "\u9080\u8BF7\u4EBAID",
          "createdAt": "\u521B\u5EFA\u65F6\u95F4",
          "updatedAt": "\u66F4\u65B0\u65F6\u95F4",
          "subscribeUrl": "\u8BA2\u9605\u94FE\u63A5",
          "telegramId": "Telegram ID"
        }
      },
      "status": {
        "normal": "\u6B63\u5E38",
        "banned": "\u5DF2\u5C01\u7981",
        "admin": "\u7BA1\u7406\u5458",
        "staff": "\u5458\u5DE5"
      }
    },
    "nav": {
      "dashboard": "\u4EEA\u8868\u76D8",
      "systemManagement": "\u7CFB\u7EDF\u7BA1\u7406",
      "systemConfig": "\u7CFB\u7EDF\u914D\u7F6E",
      "pluginManagement": "\u63D2\u4EF6\u7BA1\u7406",
      "pluginMenuDemo": "\u63D2\u4EF6\u83DC\u5355\uFF08\u6F14\u793A\uFF09",
      "noticeManagement": "\u516C\u544A\u7BA1\u7406",
      "paymentConfig": "\u652F\u4ED8\u914D\u7F6E",
      "knowledgeManagement": "\u77E5\u8BC6\u5E93\u7BA1\u7406",
      "nodeManagement": "\u8282\u70B9\u7BA1\u7406",
      "machineManagement": "\u670D\u52A1\u5668\u7BA1\u7406",
      "permissionGroupManagement": "\u6743\u9650\u7EC4\u7BA1\u7406",
      "routeManagement": "\u8DEF\u7531\u7BA1\u7406",
      "subscriptionManagement": "\u8BA2\u9605\u7BA1\u7406",
      "planManagement": "\u5957\u9910\u7BA1\u7406",
      "orderManagement": "\u8BA2\u5355\u7BA1\u7406",
      "couponManagement": "\u4F18\u60E0\u5238\u7BA1\u7406",
      "giftCardManagement": "\u793C\u54C1\u5361\u7BA1\u7406",
      "userManagement": "\u7528\u6237\u7BA1\u7406",
      "ticketManagement": "\u5DE5\u5355\u7BA1\u7406",
      "withdrawalManagement": "\u63D0\u73B0\u7BA1\u7406",
      "trafficResetLogs": "\u6D41\u91CF\u91CD\u7F6E\u65E5\u5FD7",
      "pluginApps": "\u63D2\u4EF6\u5E94\u7528"
    },
    "subscribe": {
      "plan": {
        "title": "\u8BA2\u9605\u5957\u9910",
        "add": "\u6DFB\u52A0\u5957\u9910",
        "search": "\u641C\u7D22\u5957\u9910...",
        "sort": {
          "edit": "\u7F16\u8F91\u6392\u5E8F",
          "save": "\u4FDD\u5B58\u6392\u5E8F"
        },
        "columns": {
          "id": "ID",
          "show": "\u663E\u793A",
          "sell": "\u65B0\u8D2D",
          "renew": "\u7EED\u8D39",
          "renew_tooltip": "\u5728\u8BA2\u9605\u505C\u6B62\u9500\u552E\u65F6\uFF0C\u5DF2\u8D2D\u7528\u6237\u662F\u5426\u53EF\u4EE5\u7EED\u8D39",
          "name": "\u540D\u79F0",
          "users": "\u603B\u7528\u6237",
          "active_users": "\u6D3B\u8DC3\u7528\u6237",
          "stats": "\u7EDF\u8BA1",
          "group": "\u6743\u9650\u7EC4",
          "price": "\u4EF7\u683C",
          "actions": "\u64CD\u4F5C",
          "edit": "\u7F16\u8F91",
          "delete": "\u5220\u9664",
          "delete_confirm": {
            "title": "\u786E\u8BA4\u5220\u9664",
            "description": "\u6B64\u64CD\u4F5C\u5C06\u6C38\u4E45\u5220\u9664\u8BE5\u8BA2\u9605\uFF0C\u5220\u9664\u540E\u65E0\u6CD5\u6062\u590D\u3002\u786E\u5B9A\u8981\u7EE7\u7EED\u5417\uFF1F",
            "success": "\u5220\u9664\u6210\u529F"
          },
          "price_period": {
            "hourly": "\u65F6\u4ED8",
            "daily": "\u65E5\u4ED8",
            "monthly": "\u6708\u4ED8",
            "quarterly": "\u5B63\u4ED8",
            "half_yearly": "\u534A\u5E74\u4ED8",
            "yearly": "\u5E74\u4ED8",
            "two_yearly": "\u4E24\u5E74\u4ED8",
            "three_yearly": "\u4E09\u5E74\u4ED8",
            "onetime": "\u6D41\u91CF\u5305",
            "reset_traffic": "\u91CD\u7F6E\u5305",
            "no_price": "\u65E0\u4EF7\u683C",
            "unit": {
              "hour": "\u5143/\u65F6",
              "day": "\u5143/\u5929",
              "month": "\u5143/\u6708",
              "quarter": "\u5143/\u5B63",
              "half_year": "\u5143/\u534A\u5E74",
              "year": "\u5143/\u5E74",
              "two_year": "\u5143/\u4E24\u5E74",
              "three_year": "\u5143/\u4E09\u5E74",
              "times": "\u5143/\u6B21"
            }
          }
        },
        "form": {
          "add_title": "\u6DFB\u52A0\u5957\u9910",
          "edit_title": "\u7F16\u8F91\u5957\u9910",
          "name": {
            "label": "\u5957\u9910\u540D\u79F0",
            "placeholder": "\u8BF7\u8F93\u5165\u5957\u9910\u540D\u79F0",
            "required": "\u8BF7\u8F93\u5165\u5957\u9910\u540D\u79F0"
          },
          "group": {
            "label": "\u6743\u9650\u7EC4",
            "add": "\u6DFB\u52A0\u6743\u9650\u7EC4",
            "placeholder": "\u8BF7\u9009\u62E9\u6743\u9650\u7EC4",
            "none": "\u4E0D\u7ED1\u5B9A\u6743\u9650\u7EC4"
          },
          "transfer": {
            "label": "\u6D41\u91CF",
            "placeholder": "\u8BF7\u8F93\u5165\u6D41\u91CF\u9650\u5236",
            "unit": "GB",
            "hint": "\u5957\u9910\u5305\u542B\u7684\u6D41\u91CF\u914D\u989D\uFF0C\u5355\u4F4D\u4E3A GB"
          },
          "speed": {
            "label": "\u901F\u5EA6\u9650\u5236",
            "placeholder": "0 \u8868\u793A\u4E0D\u9650\u5236",
            "unit": "Mbps",
            "hint": "\u5355\u7528\u6237\u9650\u901F\uFF0C0 \u6216\u7559\u7A7A\u8868\u793A\u4E0D\u9650\u5236"
          },
          "price": {
            "title": "\u4EF7\u683C\u8BBE\u7F6E",
            "base_price": "\u57FA\u7840\u4EF7\u683C",
            "clear": {
              "button": "\u6E05\u7A7A",
              "tooltip": "\u6E05\u7A7A\u6240\u6709\u4EF7\u683C"
            },
            "period": {
              "monthly": "\u6BCF\u6708",
              "months": "{{count}}\u4E2A\u6708"
            },
            "onetime_desc": "\u4E00\u6B21\u6027\u6D41\u91CF\u5305\uFF0C\u65E0\u65F6\u95F4\u9650\u5236",
            "reset_desc": "\u91CD\u7F6E\u6D41\u91CF\u5305\uFF0C\u53EF\u591A\u6B21\u4F7F\u7528"
          },
          "device": {
            "label": "\u8BBE\u5907\u9650\u5236",
            "placeholder": "0 \u8868\u793A\u4E0D\u9650\u5236",
            "unit": "\u53F0",
            "hint": "\u540C\u65F6\u5728\u7EBF\u8BBE\u5907\u6570\uFF0C0 \u6216\u7559\u7A7A\u8868\u793A\u4E0D\u9650\u5236"
          },
          "capacity": {
            "label": "\u5BB9\u91CF\u9650\u5236",
            "placeholder": "0 \u8868\u793A\u4E0D\u9650\u5236",
            "unit": "\u4EBA",
            "hint": "\u53EF\u552E\u5356\u540D\u989D\u4E0A\u9650\uFF0C0 \u6216\u7559\u7A7A\u8868\u793A\u4E0D\u9650\u5236"
          },
          "tags": {
            "label": "\u6807\u7B7E",
            "placeholder": "\u8F93\u5165\u6807\u7B7E\u540E\u6309\u56DE\u8F66\u786E\u8BA4"
          },
          "reset_method": {
            "label": "\u6D41\u91CF\u91CD\u7F6E\u65B9\u5F0F",
            "placeholder": "\u8BF7\u9009\u62E9\u91CD\u7F6E\u65B9\u5F0F",
            "description": "\u6D41\u91CF\u91CD\u7F6E\u65B9\u5F0F\u5C06\u51B3\u5B9A\u5982\u4F55\u91CD\u7F6E\u6D41\u91CF",
            "options": {
              "follow_system": "\u8DDF\u968F\u7CFB\u7EDF\u8BBE\u7F6E",
              "monthly_first": "\u6BCF\u6708\u9996\u65E5",
              "monthly_reset": "\u6BCF\u6708\u8D2D\u4E70\u65E5",
              "no_reset": "\u4E0D\u91CD\u7F6E",
              "yearly_first": "\u6BCF\u5E74\u9996\u65E5",
              "yearly_reset": "\u6BCF\u5E74\u8D2D\u4E70\u65E5"
            }
          },
          "content": {
            "label": "\u5957\u9910\u8BF4\u660E",
            "placeholder": "\u8BF7\u8F93\u5165\u5957\u9910\u8BF4\u660E",
            "description": "\u652F\u6301 Markdown \u683C\u5F0F",
            "preview": "\u9884\u89C8",
            "preview_button": {
              "show": "\u663E\u793A\u9884\u89C8",
              "hide": "\u9690\u85CF\u9884\u89C8"
            },
            "template": {
              "button": "\u4F7F\u7528\u6A21\u677F",
              "tooltip": "\u4F7F\u7528\u9ED8\u8BA4\u6A21\u677F",
              "content": "## \u5957\u9910\u8BE6\u60C5\n\n- \u6D41\u91CF\uFF1A{{transfer}} GB\n- \u901F\u5EA6\u9650\u5236\uFF1A{{speed}} Mbps\n- \u540C\u65F6\u5728\u7EBF\u8BBE\u5907\uFF1A{{devices}} \u53F0\n\n## \u670D\u52A1\u8BF4\u660E\n\n1. \u6D41\u91CF{{reset_method}}\u91CD\u7F6E\n2. \u652F\u6301\u591A\u5E73\u53F0\u4F7F\u7528\n3. 7\xD724\u5C0F\u65F6\u6280\u672F\u652F\u6301"
            }
          },
          "section": {
            "basic": "\u57FA\u672C\u4FE1\u606F",
            "limits": "\u914D\u989D\u4E0E\u9650\u5236",
            "content": "\u5957\u9910\u8BF4\u660E",
            "status": "\u4E0A\u67B6\u72B6\u6001"
          },
          "status": {
            "show_desc": "\u5728\u524D\u53F0\u5957\u9910\u5217\u8868\u4E2D\u5C55\u793A",
            "sell_desc": "\u5141\u8BB8\u65B0\u7528\u6237\u8D2D\u4E70\u6B64\u5957\u9910",
            "renew_desc": "\u5141\u8BB8\u5DF2\u6709\u7528\u6237\u7EED\u8D39\u6B64\u5957\u9910"
          },
          "force_update": {
            "label": "\u5F3A\u5236\u66F4\u65B0\u7528\u6237\u5957\u9910",
            "description": "\u5F00\u542F\u540E\u4FDD\u5B58\u65F6\u4F1A\u5C06\u8BE5\u5957\u9910\u4E0B\u7528\u6237\u7684\u6743\u9650\u7EC4\u3001\u6D41\u91CF\u3001\u9650\u901F\u4E0E\u8BBE\u5907\u6570\u5F3A\u5236\u540C\u6B65\u5230\u5F53\u524D\u503C\uFF0C\u4EC5\u5728\u7F16\u8F91\u5DF2\u6709\u5957\u9910\u65F6\u751F\u6548\u3002"
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
              "validation": "\u8868\u5355\u6821\u9A8C\u5931\u8D25\uFF0C\u8BF7\u68C0\u67E5\u5E76\u4FEE\u6B63\u9519\u8BEF\u540E\u91CD\u8BD5\u3002"
            }
          }
        },
        "page": {
          "description": "\u5728\u8FD9\u91CC\u53EF\u4EE5\u914D\u7F6E\u8BA2\u9605\u8BA1\u5212\uFF0C\u5305\u62EC\u6DFB\u52A0\u3001\u5220\u9664\u3001\u7F16\u8F91\u7B49\u64CD\u4F5C\u3002"
        }
      }
    },
    "settings": {
      "title": "\u7CFB\u7EDF\u8BBE\u7F6E",
      "description": "\u7BA1\u7406\u7CFB\u7EDF\u6838\u5FC3\u914D\u7F6E\uFF0C\u5305\u62EC\u7AD9\u70B9\u3001\u5B89\u5168\u3001\u8BA2\u9605\u3001\u9080\u8BF7\u4F63\u91D1\u3001\u8282\u70B9\u3001\u90AE\u4EF6\u548C\u901A\u77E5\u7B49\u8BBE\u7F6E",
      "site": {
        "title": "\u7AD9\u70B9\u8BBE\u7F6E",
        "description": "\u914D\u7F6E\u7AD9\u70B9\u57FA\u672C\u4FE1\u606F\uFF0C\u5305\u62EC\u7AD9\u70B9\u540D\u79F0\u3001\u63CF\u8FF0\u3001\u8D27\u5E01\u5355\u4F4D\u7B49\u6838\u5FC3\u8BBE\u7F6E\u3002",
        "form": {
          "siteName": {
            "label": "\u7AD9\u70B9\u540D\u79F0",
            "placeholder": "\u8BF7\u8F93\u5165\u7AD9\u70B9\u540D\u79F0",
            "description": "\u7528\u4E8E\u663E\u793A\u9700\u8981\u7AD9\u70B9\u540D\u79F0\u7684\u5730\u65B9\u3002"
          },
          "logo": {
            "label": "\u7AD9\u70B9 LOGO",
            "placeholder": "\u8BF7\u8F93\u5165LOGO URL\uFF0C\u672B\u5C3E\u4E0D\u8981/",
            "description": "\u7528\u4E8E\u663E\u793A\u9700\u8981LOGO\u7684\u5730\u65B9\u3002"
          },
          "siteDescription": {
            "label": "\u7AD9\u70B9\u63CF\u8FF0",
            "placeholder": "\u8BF7\u8F93\u5165\u7AD9\u70B9\u63CF\u8FF0",
            "description": "\u7528\u4E8E\u663E\u793A\u9700\u8981\u7AD9\u70B9\u63CF\u8FF0\u7684\u5730\u65B9\u3002"
          },
          "siteUrl": {
            "label": "\u7AD9\u70B9\u7F51\u5740",
            "placeholder": "\u8BF7\u8F93\u5165\u7AD9\u70B9URL\uFF0C\u672B\u5C3E\u4E0D\u8981/",
            "description": "\u5F53\u524D\u7F51\u7AD9\u6700\u65B0\u7F51\u5740\uFF0C\u5C06\u4F1A\u5728\u90AE\u4EF6\u7B49\u9700\u8981\u7528\u4E8E\u7F51\u5740\u5904\u4F53\u73B0\u3002"
          },
          "forceHttps": {
            "label": "\u5F3A\u5236HTTPS",
            "description": "\u5F53\u7AD9\u70B9\u6CA1\u6709\u4F7F\u7528HTTPS\uFF0CCDN\u6216\u53CD\u4EE3\u5F00\u542F\u5F3A\u5236HTTPS\u65F6\u9700\u8981\u5F00\u542F\u3002"
          },
          "maintenanceMode": {
            "label": "\u7EF4\u62A4\u6A21\u5F0F",
            "description": "\u5F00\u542F\u540E\u666E\u901A\u7528\u6237\u7684\u72B6\u6001\u53D8\u66F4\u548C\u4EE3\u7406\u6D41\u91CF\u5C06\u6682\u505C\uFF0C\u7BA1\u7406\u5458\u4ECD\u53EF\u8FDB\u5165\u540E\u53F0\u5173\u95ED\u7EF4\u62A4\u6A21\u5F0F\u3002"
          },
          "subscribeUrl": {
            "label": "\u8BA2\u9605URL",
            "placeholder": "\u7528\u4E8E\u8BA2\u9605\u6240\u4F7F\u7528\uFF0C\u591A\u4E2A\u8BA2\u9605\u5730\u5740\u7528','\u9694\u5F00.\u7559\u7A7A\u5219\u4E3A\u7AD9\u70B9URL\u3002",
            "description": "\u7528\u4E8E\u8BA2\u9605\u6240\u4F7F\u7528\uFF0C\u7559\u7A7A\u5219\u4E3A\u7AD9\u70B9URL\u3002"
          },
          "tosUrl": {
            "label": "\u7528\u6237\u6761\u6B3E(TOS)URL",
            "placeholder": "\u8BF7\u8F93\u5165\u7528\u6237\u6761\u6B3EURL\uFF0C\u672B\u5C3E\u4E0D\u8981/",
            "description": "\u7528\u4E8E\u8DF3\u8F6C\u5230\u7528\u6237\u6761\u6B3E(TOS)"
          },
          "stopRegister": {
            "label": "\u505C\u6B62\u65B0\u7528\u6237\u6CE8\u518C",
            "description": "\u5F00\u542F\u540E\u4EFB\u4F55\u4EBA\u90FD\u5C06\u65E0\u6CD5\u8FDB\u884C\u6CE8\u518C\u3002"
          },
          "ticketMustWaitReply": {
            "label": "\u5DE5\u5355\u7B49\u5F85\u56DE\u590D\u9650\u5236",
            "description": "\u5F00\u542F\u540E\uFF0C\u7528\u6237\u5728\u7BA1\u7406\u5458\u56DE\u590D\u524D\u65E0\u6CD5\u5728\u540C\u4E00\u5DE5\u5355\u5185\u8FDE\u7EED\u53D1\u9001\u6D88\u606F\u3002"
          },
          "tryOut": {
            "label": "\u6CE8\u518C\u8BD5\u7528\u5957\u9910",
            "placeholder": "\u8BF7\u9009\u62E9\u5957\u9910",
            "description": "\u9009\u62E9\u9700\u8981\u8BD5\u7528\u7684\u8BA2\u9605\uFF0C\u5982\u679C\u6CA1\u6709\u9009\u9879\u8BF7\u5148\u524D\u5F80\u8BA2\u9605\u7BA1\u7406\u6DFB\u52A0\u3002",
            "no_plan": "\u5173\u95ED",
            "duration": {
              "label": "\u6CE8\u518C\u8BD5\u7528\u65F6\u957F",
              "placeholder": "0",
              "description": "\u6CE8\u518C\u8BD5\u7528\u65F6\u957F\uFF0C\u5355\u4F4D\u4E3A\u5C0F\u65F6\u3002"
            }
          },
          "currency": {
            "label": "\u8D27\u5E01\u5355\u4F4D",
            "placeholder": "CNY",
            "description": "\u4EC5\u7528\u4E8E\u5C55\u793A\u4F7F\u7528\uFF0C\u66F4\u6539\u540E\u7CFB\u7EDF\u4E2D\u6240\u6709\u7684\u8D27\u5E01\u5355\u4F4D\u90FD\u5C06\u53D1\u751F\u53D8\u66F4\u3002"
          },
          "currencySymbol": {
            "label": "\u8D27\u5E01\u7B26\u53F7",
            "placeholder": "\xA5",
            "description": "\u4EC5\u7528\u4E8E\u5C55\u793A\u4F7F\u7528\uFF0C\u66F4\u6539\u540E\u7CFB\u7EDF\u4E2D\u6240\u6709\u7684\u8D27\u5E01\u5355\u4F4D\u90FD\u5C06\u53D1\u751F\u53D8\u66F4\u3002"
          }
        }
      },
      "safe": {
        "title": "\u5B89\u5168\u8BBE\u7F6E",
        "description": "\u914D\u7F6E\u7CFB\u7EDF\u5B89\u5168\u76F8\u5173\u9009\u9879\uFF0C\u5305\u62EC\u767B\u5F55\u9A8C\u8BC1\u3001\u5BC6\u7801\u7B56\u7565\u3001API\u8BBF\u95EE\u7B49\u5B89\u5168\u8BBE\u7F6E\u3002",
        "form": {
          "emailVerify": {
            "label": "\u90AE\u7BB1\u9A8C\u8BC1",
            "description": "\u5F00\u542F\u540E\u5C06\u4F1A\u5F3A\u5236\u8981\u6C42\u7528\u6237\u8FDB\u884C\u90AE\u7BB1\u9A8C\u8BC1\u3002"
          },
          "gmailLimit": {
            "label": "\u7981\u6B62\u4F7F\u7528Gmail\u591A\u522B\u540D",
            "description": "\u5F00\u542F\u540EGmail\u591A\u522B\u540D\u5C06\u65E0\u6CD5\u6CE8\u518C\u3002"
          },
          "safeMode": {
            "label": "\u5B89\u5168\u6A21\u5F0F",
            "description": "\u5F00\u542F\u540E\u9664\u4E86\u7AD9\u70B9URL\u4EE5\u5916\u7684\u7ED1\u5B9A\u672C\u7AD9\u70B9\u7684\u57DF\u540D\u8BBF\u95EE\u90FD\u5C06\u4F1A\u88AB403\u3002"
          },
          "securePath": {
            "label": "\u540E\u53F0\u8DEF\u5F84",
            "placeholder": "admin",
            "description": "\u540E\u53F0\u7BA1\u7406\u8DEF\u5F84\uFF0C\u4FEE\u6539\u540E\u5C06\u4F1A\u6539\u53D8\u539F\u6709\u7684admin\u8DEF\u5F84"
          },
          "emailWhitelist": {
            "label": "\u90AE\u7BB1\u540E\u7F00\u767D\u540D\u5355",
            "description": "\u5F00\u542F\u540E\u5728\u540D\u5355\u4E2D\u7684\u90AE\u7BB1\u540E\u7F00\u624D\u5141\u8BB8\u8FDB\u884C\u6CE8\u518C\u3002",
            "suffixes": {
              "label": "\u90AE\u7BB1\u540E\u7F00",
              "placeholder": "\u8F93\u5165\u90AE\u7BB1\u540E\u7F00\uFF0C\u6BCF\u884C\u4E00\u4E2A",
              "description": "\u8F93\u5165\u5141\u8BB8\u7684\u90AE\u7BB1\u540E\u7F00\uFF0C\u6BCF\u884C\u4E00\u4E2A"
            }
          },
          "captcha": {
            "enable": {
              "label": "\u542F\u7528\u9A8C\u8BC1\u7801",
              "description": "\u5F00\u542F\u540E\u7528\u6237\u6CE8\u518C\u65F6\u9700\u8981\u901A\u8FC7\u9A8C\u8BC1\u7801\u9A8C\u8BC1\u3002"
            },
            "type": {
              "label": "\u9A8C\u8BC1\u7801\u7C7B\u578B",
              "description": "\u9009\u62E9\u8981\u4F7F\u7528\u7684\u9A8C\u8BC1\u7801\u670D\u52A1\u7C7B\u578B",
              "options": {
                "recaptcha": "Google reCAPTCHA v2",
                "recaptcha-v3": "Google reCAPTCHA v3",
                "turnstile": "Cloudflare Turnstile"
              }
            },
            "recaptcha": {
              "key": {
                "label": "reCAPTCHA\u5BC6\u94A5",
                "placeholder": "\u8F93\u5165reCAPTCHA\u5BC6\u94A5",
                "description": "\u8F93\u5165\u60A8\u7684reCAPTCHA\u5BC6\u94A5"
              },
              "siteKey": {
                "label": "reCAPTCHA\u7AD9\u70B9\u5BC6\u94A5",
                "placeholder": "\u8F93\u5165reCAPTCHA\u7AD9\u70B9\u5BC6\u94A5",
                "description": "\u8F93\u5165\u60A8\u7684reCAPTCHA\u7AD9\u70B9\u5BC6\u94A5"
              }
            },
            "recaptcha_v3": {
              "secretKey": {
                "label": "reCAPTCHA v3\u5BC6\u94A5",
                "placeholder": "\u8F93\u5165reCAPTCHA v3\u5BC6\u94A5",
                "description": "\u8F93\u5165\u60A8\u7684reCAPTCHA v3\u670D\u52A1\u5668\u5BC6\u94A5"
              },
              "siteKey": {
                "label": "reCAPTCHA v3\u7AD9\u70B9\u5BC6\u94A5",
                "placeholder": "\u8F93\u5165reCAPTCHA v3\u7AD9\u70B9\u5BC6\u94A5",
                "description": "\u8F93\u5165\u60A8\u7684reCAPTCHA v3\u7AD9\u70B9\u5BC6\u94A5"
              },
              "scoreThreshold": {
                "label": "\u5206\u6570\u9608\u503C",
                "placeholder": "0.5",
                "description": "\u8BBE\u7F6E\u9A8C\u8BC1\u5206\u6570\u9608\u503C\uFF080-1\uFF09\uFF0C\u5206\u6570\u8D8A\u9AD8\u8868\u793A\u8D8A\u53EF\u80FD\u662F\u771F\u4EBA\u64CD\u4F5C"
              }
            },
            "turnstile": {
              "secretKey": {
                "label": "Turnstile\u5BC6\u94A5",
                "placeholder": "\u8F93\u5165Turnstile\u5BC6\u94A5",
                "description": "\u8F93\u5165\u60A8\u7684Cloudflare Turnstile\u5BC6\u94A5"
              },
              "siteKey": {
                "label": "Turnstile\u7AD9\u70B9\u5BC6\u94A5",
                "placeholder": "\u8F93\u5165Turnstile\u7AD9\u70B9\u5BC6\u94A5",
                "description": "\u8F93\u5165\u60A8\u7684Cloudflare Turnstile\u7AD9\u70B9\u5BC6\u94A5"
              }
            }
          },
          "registerLimit": {
            "enable": {
              "label": "IP\u6CE8\u518C\u9650\u5236",
              "description": "\u5F00\u542F\u540E\u5C06\u9650\u5236\u540C\u4E00IP\u7684\u6CE8\u518C\u6B21\u6570\u3002"
            },
            "count": {
              "label": "\u6CE8\u518C\u6B21\u6570",
              "placeholder": "\u8F93\u5165\u6700\u5927\u6CE8\u518C\u6B21\u6570",
              "description": "\u540C\u4E00IP\u5141\u8BB8\u7684\u6700\u5927\u6CE8\u518C\u6B21\u6570"
            },
            "expire": {
              "label": "\u9650\u5236\u65F6\u957F",
              "placeholder": "\u8F93\u5165\u9650\u5236\u65F6\u957F\uFF08\u5206\u949F\uFF09",
              "description": "\u6CE8\u518C\u9650\u5236\u7684\u6301\u7EED\u65F6\u95F4\uFF08\u5206\u949F\uFF09"
            }
          },
          "passwordLimit": {
            "enable": {
              "label": "\u5BC6\u7801\u5C1D\u8BD5\u9650\u5236",
              "description": "\u5F00\u542F\u540E\u5C06\u9650\u5236\u5BC6\u7801\u5C1D\u8BD5\u6B21\u6570\u3002"
            },
            "count": {
              "label": "\u5C1D\u8BD5\u6B21\u6570",
              "placeholder": "\u8F93\u5165\u6700\u5927\u5C1D\u8BD5\u6B21\u6570",
              "description": "\u5141\u8BB8\u7684\u6700\u5927\u5BC6\u7801\u5C1D\u8BD5\u6B21\u6570"
            },
            "expire": {
              "label": "\u9501\u5B9A\u65F6\u957F",
              "placeholder": "\u8F93\u5165\u9501\u5B9A\u65F6\u957F\uFF08\u5206\u949F\uFF09",
              "description": "\u8D26\u6237\u9501\u5B9A\u7684\u6301\u7EED\u65F6\u95F4\uFF08\u5206\u949F\uFF09"
            }
          }
        }
      },
      "subscribe": {
        "title": "\u8BA2\u9605\u8BBE\u7F6E",
        "description": "\u7BA1\u7406\u7528\u6237\u8BA2\u9605\u76F8\u5173\u914D\u7F6E\uFF0C\u5305\u62EC\u8BA2\u9605\u94FE\u63A5\u683C\u5F0F\u3001\u66F4\u65B0\u9891\u7387\u3001\u6D41\u91CF\u7EDF\u8BA1\u7B49\u8BBE\u7F6E\u3002",
        "plan_change_enable": {
          "title": "\u5141\u8BB8\u7528\u6237\u66F4\u6539\u8BA2\u9605",
          "description": "\u5F00\u542F\u540E\u7528\u6237\u5C06\u4F1A\u53EF\u4EE5\u5BF9\u8BA2\u9605\u8BA1\u5212\u8FDB\u884C\u53D8\u66F4\u3002"
        },
        "reset_traffic_method": {
          "title": "\u6708\u6D41\u91CF\u91CD\u7F6E\u65B9\u5F0F",
          "description": "\u5168\u5C40\u6D41\u91CF\u91CD\u7F6E\u65B9\u5F0F\uFF0C\u9ED8\u8BA4\u6BCF\u67081\u53F7\u3002\u53EF\u4EE5\u5728\u8BA2\u9605\u7BA1\u7406\u4E3A\u8BA2\u9605\u5355\u72EC\u8BBE\u7F6E\u3002",
          "options": {
            "monthly_first": "\u6BCF\u67081\u53F7",
            "monthly_reset": "\u6309\u6708\u91CD\u7F6E",
            "no_reset": "\u4E0D\u91CD\u7F6E",
            "yearly_first": "\u6BCF\u5E741\u67081\u53F7",
            "yearly_reset": "\u6309\u5E74\u91CD\u7F6E"
          }
        },
        "surplus_enable": {
          "title": "\u5F00\u542F\u6298\u62B5\u65B9\u6848",
          "description": "\u5F00\u542F\u540E\u7528\u6237\u66F4\u6362\u8BA2\u9605\u5C06\u4F1A\u7531\u7CFB\u7EDF\u5BF9\u539F\u6709\u8BA2\u9605\u8FDB\u884C\u6298\u62B5\uFF0C\u65B9\u6848\u53C2\u8003\u6587\u6863\u3002"
        },
        "new_order_event": {
          "title": "\u5F53\u8BA2\u9605\u65B0\u8D2D\u65F6\u89E6\u53D1\u4E8B\u4EF6",
          "description": "\u65B0\u8D2D\u8BA2\u9605\u5B8C\u6210\u65F6\u5C06\u89E6\u53D1\u8BE5\u4EFB\u52A1\u3002",
          "options": {
            "no_action": "\u4E0D\u6267\u884C\u4EFB\u4F55\u52A8\u4F5C",
            "reset_traffic": "\u91CD\u7F6E\u7528\u6237\u6D41\u91CF"
          }
        },
        "renew_order_event": {
          "title": "\u5F53\u8BA2\u9605\u7EED\u8D39\u65F6\u89E6\u53D1\u4E8B\u4EF6",
          "description": "\u7EED\u8D39\u8BA2\u9605\u5B8C\u6210\u65F6\u5C06\u89E6\u53D1\u8BE5\u4EFB\u52A1\u3002",
          "options": {
            "no_action": "\u4E0D\u6267\u884C\u4EFB\u4F55\u52A8\u4F5C",
            "reset_traffic": "\u91CD\u7F6E\u7528\u6237\u6D41\u91CF"
          }
        },
        "change_order_event": {
          "title": "\u5F53\u8BA2\u9605\u53D8\u66F4\u65F6\u89E6\u53D1\u4E8B\u4EF6",
          "description": "\u53D8\u66F4\u8BA2\u9605\u5B8C\u6210\u65F6\u5C06\u89E6\u53D1\u8BE5\u4EFB\u52A1\u3002",
          "options": {
            "no_action": "\u4E0D\u6267\u884C\u4EFB\u4F55\u52A8\u4F5C",
            "reset_traffic": "\u91CD\u7F6E\u7528\u6237\u6D41\u91CF"
          }
        },
        "subscribe_path": {
          "title": "\u8BA2\u9605\u8DEF\u5F84",
          "description": "\u8BA2\u9605\u8DEF\u5F84\uFF0C\u4FEE\u6539\u540E\u5C06\u4F1A\u6539\u53D8\u539F\u6709\u7684subscribe\u8DEF\u5F84",
          "current_format": "\u5F53\u524D\u8BA2\u9605\u8DEF\u5F84\u683C\u5F0F\uFF1A{path}/xxxxxxxxxx",
          "restart_tip": "\u4FEE\u6539\u8BA2\u9605\u8DEF\u5F84\u540E\uFF0C\u53EF\u80FD\u9700\u8981\u91CD\u542F\u670D\u52A1\u624D\u80FD\u751F\u6548\u3002"
        },
        "show_info_to_server": {
          "title": "\u5728\u8BA2\u9605\u4E2D\u5C55\u793A\u8BA2\u9605\u4FE1\u606F",
          "description": "\u5F00\u542F\u540E\u5C06\u4F1A\u5728\u7528\u6237\u8BA2\u9605\u8282\u70B9\u65F6\u8F93\u51FA\u8BA2\u9605\u4FE1\u606F\u3002"
        },
        "show_protocol_to_server": {
          "title": "\u5728\u8BA2\u9605\u4E2D\u7EBF\u8DEF\u540D\u79F0\u4E2D\u663E\u793A\u534F\u8BAE\u540D\u79F0",
          "description": "\u5F00\u542F\u540E\u8BA2\u9605\u7EBF\u8DEF\u4F1A\u9644\u5E26\u534F\u8BAE\u540D\u79F0\uFF08\u4F8B\u5982: [Hy2]\u9999\u6E2F\uFF09"
        },
        "default_remind_expire": {
          "title": "\u65B0\u7528\u6237\u9ED8\u8BA4\u5F00\u542F\u5230\u671F\u63D0\u9192",
          "description": "\u5F00\u542F\u540E\u65B0\u6CE8\u518C\u7528\u6237\u9ED8\u8BA4\u542F\u7528\u8BA2\u9605\u5230\u671F\u63D0\u9192\uFF0C\u53EF\u5728\u7528\u6237\u7BA1\u7406\u5355\u72EC\u8C03\u6574\u3002"
        },
        "default_remind_traffic": {
          "title": "\u65B0\u7528\u6237\u9ED8\u8BA4\u5F00\u542F\u6D41\u91CF\u63D0\u9192",
          "description": "\u5F00\u542F\u540E\u65B0\u6CE8\u518C\u7528\u6237\u9ED8\u8BA4\u542F\u7528\u6D41\u91CF\u4E0D\u8DB3\u63D0\u9192\uFF0C\u53EF\u5728\u7528\u6237\u7BA1\u7406\u5355\u72EC\u8C03\u6574\u3002"
        },
        "deposit_enable": {
          "title": "\u5F00\u542F\u4F59\u989D\u5145\u503C",
          "description": "\u5F00\u542F\u540E\u7528\u6237\u53EF\u5728\u524D\u53F0\u521B\u5EFA\u4F59\u989D\u5145\u503C\u8BA2\u5355\u5E76\u5B8C\u6210\u652F\u4ED8\u5165\u8D26\u3002"
        },
        "deposit_commission_enable": {
          "title": "\u4F59\u989D\u5145\u503C\u53C2\u4E0E\u9080\u8BF7\u8FD4\u4F63",
          "description": "\u5F00\u542F\u540E\u4F59\u989D\u5145\u503C\u8BA2\u5355\u5C06\u6309\u9080\u8BF7\u89C4\u5219\u8BA1\u7B97\u4F63\u91D1\u3002"
        },
        "deposit_min_amount": {
          "title": "\u6700\u4F4E\u5145\u503C\u91D1\u989D\uFF08\u5206\uFF09",
          "description": "\u7528\u6237\u5355\u7B14\u5145\u503C\u7684\u6700\u4F4E\u91D1\u989D\uFF0C\u5355\u4F4D\u4E3A\u5206\u3002\u4F8B\u5982 100 \u8868\u793A 1 \u5143\u3002",
          "placeholder": "100"
        },
        "deposit_max_amount": {
          "title": "\u6700\u9AD8\u5145\u503C\u91D1\u989D\uFF08\u5206\uFF09",
          "description": "\u7528\u6237\u5355\u7B14\u5145\u503C\u7684\u6700\u9AD8\u91D1\u989D\uFF0C\u5355\u4F4D\u4E3A\u5206\u3002",
          "placeholder": "999999900"
        },
        "deposit_bonus": {
          "title": "\u5145\u503C\u8D60\u9001\u9636\u68AF",
          "description": "\u683C\u5F0F\u4E3A\u300C\u95E8\u69DB\u5143:\u8D60\u9001\u5143\u300D\uFF0C\u4F8B\u5982 100:10 \u8868\u793A\u5145\u503C\u6EE1 100 \u5143\u8D60\u9001 10 \u5143\u3002\u53D6\u6EE1\u8DB3\u95E8\u69DB\u7684\u6700\u5927\u8D60\u9001\u3002",
          "placeholder": "100:10"
        },
        "saving": "\u4FDD\u5B58\u4E2D...",
        "plan": {
          "title": "\u8BA2\u9605\u5957\u9910",
          "add": "\u6DFB\u52A0\u5957\u9910",
          "search": "\u641C\u7D22\u5957\u9910...",
          "sort": {
            "edit": "\u7F16\u8F91\u6392\u5E8F",
            "save": "\u4FDD\u5B58\u6392\u5E8F"
          },
          "columns": {
            "id": "\u7F16\u53F7",
            "show": "\u663E\u793A",
            "sell": "\u65B0\u8D2D",
            "renew": "\u7EED\u8D39",
            "renew_tooltip": "\u5728\u8BA2\u9605\u505C\u6B62\u9500\u552E\u65F6\uFF0C\u5DF2\u8D2D\u7528\u6237\u662F\u5426\u53EF\u4EE5\u7EED\u8D39",
            "name": "\u540D\u79F0",
            "stats": "\u7EDF\u8BA1",
            "group": "\u6743\u9650\u7EC4",
            "price": "\u4EF7\u683C",
            "actions": "\u64CD\u4F5C",
            "edit": "\u7F16\u8F91",
            "delete": "\u5220\u9664",
            "delete_confirm": {
              "title": "\u786E\u8BA4\u5220\u9664",
              "description": "\u6B64\u64CD\u4F5C\u5C06\u6C38\u4E45\u5220\u9664\u8BE5\u8BA2\u9605\uFF0C\u5220\u9664\u540E\u65E0\u6CD5\u6062\u590D\u3002\u786E\u5B9A\u8981\u7EE7\u7EED\u5417\uFF1F",
              "success": "\u5220\u9664\u6210\u529F"
            },
            "price_period": {
              "hourly": "\u65F6\u4ED8",
              "daily": "\u65E5\u4ED8",
              "monthly": "\u6708\u4ED8",
              "quarterly": "\u5B63\u4ED8",
              "half_yearly": "\u534A\u5E74\u4ED8",
              "yearly": "\u5E74\u4ED8",
              "two_yearly": "\u4E24\u5E74\u4ED8",
              "three_yearly": "\u4E09\u5E74\u4ED8",
              "onetime": "\u6D41\u91CF\u5305",
              "reset_traffic": "\u91CD\u7F6E\u5305",
              "unit": {
                "hour": "\u5143/\u65F6",
                "day": "\u5143/\u5929",
                "month": "\u5143/\u6708",
                "quarter": "\u5143/\u5B63",
                "half_year": "\u5143/\u534A\u5E74",
                "year": "\u5143/\u5E74",
                "two_year": "\u5143/\u4E24\u5E74",
                "three_year": "\u5143/\u4E09\u5E74",
                "times": "\u5143/\u6B21"
              }
            }
          },
          "form": {
            "add_title": "\u6DFB\u52A0\u5957\u9910",
            "edit_title": "\u7F16\u8F91\u5957\u9910",
            "name": {
              "label": "\u5957\u9910\u540D\u79F0",
              "placeholder": "\u8BF7\u8F93\u5165\u5957\u9910\u540D\u79F0"
            },
            "group": {
              "label": "\u6743\u9650\u7EC4",
              "placeholder": "\u9009\u62E9\u6743\u9650\u7EC4",
              "add": "\u6DFB\u52A0\u6743\u9650\u7EC4",
              "none": "\u4E0D\u7ED1\u5B9A\u6743\u9650\u7EC4"
            },
            "transfer": {
              "label": "\u6D41\u91CF",
              "placeholder": "\u8BF7\u8F93\u5165\u6D41\u91CF\u5927\u5C0F",
              "unit": "GB"
            },
            "speed": {
              "label": "\u9650\u901F",
              "placeholder": "\u8BF7\u8F93\u5165\u9650\u901F",
              "unit": "Mbps"
            },
            "price": {
              "title": "\u552E\u4EF7\u8BBE\u7F6E",
              "base_price": "\u57FA\u7840\u6708\u4ED8\u4EF7\u683C",
              "clear": {
                "button": "\u6E05\u7A7A\u4EF7\u683C",
                "tooltip": "\u6E05\u7A7A\u6240\u6709\u5468\u671F\u7684\u4EF7\u683C\u8BBE\u7F6E"
              }
            },
            "device": {
              "label": "\u8BBE\u5907\u9650\u5236",
              "placeholder": "\u7559\u7A7A\u5219\u4E0D\u9650\u5236",
              "unit": "\u53F0"
            },
            "capacity": {
              "label": "\u5BB9\u91CF\u9650\u5236",
              "placeholder": "\u7559\u7A7A\u5219\u4E0D\u9650\u5236",
              "unit": "\u4EBA"
            },
            "reset_method": {
              "label": "\u6D41\u91CF\u91CD\u7F6E\u65B9\u5F0F",
              "placeholder": "\u9009\u62E9\u6D41\u91CF\u91CD\u7F6E\u65B9\u5F0F",
              "description": "\u8BBE\u7F6E\u8BA2\u9605\u6D41\u91CF\u7684\u91CD\u7F6E\u65B9\u5F0F\uFF0C\u4E0D\u540C\u7684\u91CD\u7F6E\u65B9\u5F0F\u4F1A\u5F71\u54CD\u7528\u6237\u7684\u6D41\u91CF\u8BA1\u7B97\u65B9\u5F0F",
              "options": {
                "follow_system": "\u8DDF\u968F\u7CFB\u7EDF\u8BBE\u7F6E",
                "monthly_first": "\u6BCF\u67081\u53F7",
                "monthly_reset": "\u6309\u6708\u91CD\u7F6E",
                "no_reset": "\u4E0D\u91CD\u7F6E",
                "yearly_first": "\u6BCF\u5E741\u67081\u65E5",
                "yearly_reset": "\u6309\u5E74\u91CD\u7F6E"
              }
            },
            "content": {
              "label": "\u5957\u9910\u63CF\u8FF0",
              "placeholder": "\u5728\u8FD9\u91CC\u7F16\u5199\u5957\u9910\u63CF\u8FF0...",
              "description": "\u652F\u6301 Markdown \u683C\u5F0F\uFF0C\u53EF\u4EE5\u4F7F\u7528\u6807\u9898\u3001\u5217\u8868\u3001\u7C97\u4F53\u3001\u659C\u4F53\u7B49\u6837\u5F0F\u6765\u7F8E\u5316\u63CF\u8FF0\u5185\u5BB9",
              "preview": "\u9884\u89C8",
              "preview_button": {
                "show": "\u663E\u793A\u9884\u89C8",
                "hide": "\u9690\u85CF\u9884\u89C8"
              },
              "template": {
                "button": "\u4F7F\u7528\u6A21\u677F",
                "tooltip": "\u70B9\u51FB\u4F7F\u7528\u9884\u8BBE\u7684\u5957\u9910\u63CF\u8FF0\u6A21\u677F",
                "content": "## \u5957\u9910\u7279\u70B9\n\u2022 \u9AD8\u901F\u7A33\u5B9A\u7684\u5168\u7403\u7F51\u7EDC\u63A5\u5165\n\u2022 \u652F\u6301\u591A\u8BBE\u5907\u540C\u65F6\u5728\u7EBF\n\u2022 \u65E0\u9650\u5236\u7684\u6D41\u91CF\u91CD\u7F6E\n\n## \u4F7F\u7528\u8BF4\u660E\n1. \u652F\u6301\u8BBE\u5907\uFF1AiOS\u3001Android\u3001Windows\u3001macOS\n2. 24/7 \u6280\u672F\u652F\u6301\n3. \u81EA\u52A8\u5B9A\u671F\u6D41\u91CF\u91CD\u7F6E\n\n## \u6CE8\u610F\u4E8B\u9879\n- \u7981\u6B62\u6EE5\u7528\n- \u9075\u5B88\u5F53\u5730\u6CD5\u5F8B\u6CD5\u89C4\n- \u652F\u6301\u968F\u65F6\u66F4\u6362\u5957\u9910"
              }
            },
            "force_update": {
              "label": "\u5F3A\u5236\u66F4\u65B0\u5230\u7528\u6237",
              "description": "\u5F00\u542F\u540E\u4FDD\u5B58\u65F6\u5C06\u8BE5\u5957\u9910\u4E0B\u7528\u6237\u7684\u6743\u9650\u7EC4\u3001\u6D41\u91CF\u3001\u9650\u901F\u4E0E\u8BBE\u5907\u6570\u5F3A\u5236\u540C\u6B65\u5230\u5F53\u524D\u503C\u3002"
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
            "description": "\u5728\u8FD9\u91CC\u53EF\u4EE5\u914D\u7F6E\u8BA2\u9605\u8BA1\u5212\uFF0C\u5305\u62EC\u6DFB\u52A0\u3001\u5220\u9664\u3001\u7F16\u8F91\u7B49\u64CD\u4F5C\u3002"
          }
        }
      },
      "email": {
        "title": "\u90AE\u4EF6\u8BBE\u7F6E",
        "description": "\u914D\u7F6E\u7CFB\u7EDF\u90AE\u4EF6\u670D\u52A1\uFF0C\u7528\u4E8E\u53D1\u9001\u9A8C\u8BC1\u7801\u3001\u5BC6\u7801\u91CD\u7F6E\u3001\u901A\u77E5\u7B49\u90AE\u4EF6\uFF0C\u652F\u6301\u591A\u79CDSMTP\u670D\u52A1\u5546\u3002",
        "tab_settings": "\u57FA\u672C\u8BBE\u7F6E",
        "tab_templates": "\u6A21\u677F\u7BA1\u7406",
        "email_host": {
          "title": "SMTP\u4E3B\u673A",
          "description": "SMTP\u670D\u52A1\u5668\u5730\u5740\uFF0C\u4F8B\u5982\uFF1Asmtp.gmail.com",
          "placeholder": "smtp.example.com"
        },
        "email_port": {
          "title": "SMTP\u7AEF\u53E3",
          "description": "SMTP\u670D\u52A1\u5668\u7AEF\u53E3\uFF0C\u5E38\u7528\u7AEF\u53E3\uFF1A25, 465, 587",
          "placeholder": "465"
        },
        "email_username": {
          "title": "SMTP\u7528\u6237\u540D",
          "description": "SMTP\u8BA4\u8BC1\u7528\u6237\u540D",
          "placeholder": "user@example.com"
        },
        "email_password": {
          "title": "SMTP\u5BC6\u7801",
          "description": "SMTP\u8BA4\u8BC1\u5BC6\u7801\u6216\u5E94\u7528\u4E13\u7528\u5BC6\u7801",
          "placeholder": "\u8BF7\u8F93\u5165\u5BC6\u7801\u6216\u5E94\u7528\u4E13\u7528\u5BC6\u7801"
        },
        "email_encryption": {
          "title": "\u52A0\u5BC6\u65B9\u5F0F",
          "description": "\u90AE\u4EF6\u52A0\u5BC6\u65B9\u5F0F",
          "placeholder": "\u9009\u62E9\u52A0\u5BC6\u65B9\u5F0F",
          "none": "\u65E0",
          "ssl": "SSL/TLS",
          "tls": "STARTTLS"
        },
        "email_from_address": {
          "title": "\u53D1\u4EF6\u4EBA\u5730\u5740",
          "description": "\u53D1\u4EF6\u4EBA\u90AE\u7BB1\u5730\u5740",
          "placeholder": "noreply@example.com"
        },
        "email_from_name": {
          "title": "\u53D1\u4EF6\u4EBA\u540D\u79F0",
          "description": "\u53D1\u4EF6\u4EBA\u663E\u793A\u540D\u79F0"
        },
        "email_template": {
          "title": "\u90AE\u4EF6\u6A21\u677F",
          "description": "\u81EA\u5B9A\u4E49\u90AE\u4EF6\u6A21\u677F\u65B9\u5F0F\u8BF7\u67E5\u770B\u6587\u6863",
          "placeholder": "\u9009\u62E9\u90AE\u4EF6\u6A21\u677F"
        },
        "remind_mail_enable": {
          "title": "\u90AE\u4EF6\u63D0\u9192",
          "description": "\u5F00\u542F\u540E\u7528\u6237\u8BA2\u9605\u5373\u5C06\u5230\u671F\u6216\u6D41\u91CF\u4E0D\u8DB3\u65F6\u4F1A\u6536\u5230\u90AE\u4EF6\u901A\u77E5\u3002"
        },
        "test": {
          "title": "\u53D1\u9001\u6D4B\u8BD5\u90AE\u4EF6",
          "sending": "\u53D1\u9001\u4E2D...",
          "description": "\u53D1\u9001\u6D4B\u8BD5\u90AE\u4EF6\u4EE5\u9A8C\u8BC1\u914D\u7F6E",
          "success": "\u6D4B\u8BD5\u90AE\u4EF6\u53D1\u9001\u6210\u529F",
          "error": "\u6D4B\u8BD5\u90AE\u4EF6\u53D1\u9001\u5931\u8D25",
          "label_to": "\u6536\u4EF6\u4EBA",
          "label_subject": "\u4E3B\u9898",
          "placeholder_to": "user@example.com",
          "placeholder_subject": "\u6D4B\u8BD5\u90AE\u4EF6\u4E3B\u9898"
        }
      },
      "telegram": {
        "title": "Telegram\u8BBE\u7F6E",
        "description": "\u914D\u7F6ETelegram\u673A\u5668\u4EBA\u529F\u80FD\uFF0C\u5B9E\u73B0\u7528\u6237\u901A\u77E5\u3001\u8D26\u6237\u7ED1\u5B9A\u3001\u6307\u4EE4\u4EA4\u4E92\u7B49\u81EA\u52A8\u5316\u670D\u52A1\u3002",
        "bot_token": {
          "title": "\u673A\u5668\u4EBA\u4EE4\u724C",
          "description": "\u8BF7\u8F93\u5165\u4ECEBotfather\u83B7\u53D6\u7684\u4EE4\u724C\u3002",
          "placeholder": "0000000000:xxxxxxxxx_xxxxxxxxxxxxxxx"
        },
        "webhook_url": {
          "title": "Webhook \u57FA\u7840\u5730\u5740",
          "description": "\u8FD9\u91CC\u53EA\u586B\u5199\u57FA\u7840\u5730\u5740\uFF0C\u7CFB\u7EDF\u4F1A\u81EA\u52A8\u62FC\u63A5 Telegram \u7684\u5B8C\u6574 Webhook \u56DE\u8C03\u8DEF\u5F84\u3002\u7559\u7A7A\u65F6\u9ED8\u8BA4\u4F7F\u7528\u7AD9\u70B9\u7F51\u5740\u3002",
          "docs": "\u67E5\u770B Telegram Webhook \u6587\u6863",
          "placeholder": "https://example.com"
        },
        "webhook": {
          "title": "\u8BBE\u7F6EWebhook",
          "description": "\u8BBE\u7F6E\u673A\u5668\u4EBA\u7684webhook\uFF0C\u4E0D\u8BBE\u7F6E\u5C06\u65E0\u6CD5\u6536\u5230Telegram\u901A\u77E5\u3002",
          "button": "\u4E00\u952E\u8BBE\u7F6E",
          "setting": "\u8BBE\u7F6E\u4E2D...",
          "success": "Webhook \u8BBE\u7F6E\u6210\u529F",
          "error": "Webhook \u8BBE\u7F6E\u5931\u8D25",
          "target_default": "\u5F53\u524D\u5C06\u4F7F\u7528\u7AD9\u70B9\u7F51\u5740\u4F5C\u4E3A Webhook Base URL\u3002",
          "target_custom": "\u5F53\u524D\u5C06\u4F7F\u7528\u81EA\u5B9A\u4E49 Webhook Base URL\uFF1A{{url}}",
          "debug": {
            "title": "Webhook \u8C03\u8BD5\u4FE1\u606F",
            "success": "\u6210\u529F\u72B6\u6001",
            "url": "Webhook \u5730\u5740",
            "baseUrl": "\u57FA\u7840\u5730\u5740"
          }
        },
        "bot_enable": {
          "title": "\u542F\u7528Telegram\u7ED1\u5B9A\u5F15\u5BFC",
          "description": "\u5F00\u542F\u540E\u5C06\u5728\u7528\u6237\u7AEF\u663E\u793ATelegram\u7ED1\u5B9A\u5F15\u5BFC\uFF0C\u5E2E\u52A9\u7528\u6237\u7ED1\u5B9ATelegram\u8D26\u6237\u4EE5\u63A5\u6536\u901A\u77E5\u3002"
        },
        "discuss_link": {
          "title": "\u7FA4\u7EC4\u94FE\u63A5",
          "description": "\u586B\u5199\u540E\u5C06\u5728\u7528\u6237\u7AEF\u663E\u793A\u6216\u5728\u9700\u8981\u7684\u5730\u65B9\u4F7F\u7528\u3002",
          "placeholder": "https://t.me/xxxxxx"
        }
      },
      "app": {
        "title": "APP\u8BBE\u7F6E",
        "description": "\u7BA1\u7406\u79FB\u52A8\u5E94\u7528\u7A0B\u5E8F\u76F8\u5173\u914D\u7F6E\uFF0C\u5305\u62ECAPI\u63A5\u53E3\u3001\u7248\u672C\u63A7\u5236\u3001\u63A8\u9001\u901A\u77E5\u7B49\u529F\u80FD\u8BBE\u7F6E\u3002",
        "common": {
          "placeholder": "\u8BF7\u8F93\u5165"
        },
        "windows": {
          "version": {
            "title": "Windows\u7248\u672C",
            "description": "Windows\u5BA2\u6237\u7AEF\u5F53\u524D\u7248\u672C\u53F7"
          },
          "download": {
            "title": "Windows\u4E0B\u8F7D\u5730\u5740",
            "description": "Windows\u5BA2\u6237\u7AEF\u4E0B\u8F7D\u94FE\u63A5"
          }
        },
        "macos": {
          "version": {
            "title": "macOS\u7248\u672C",
            "description": "macOS\u5BA2\u6237\u7AEF\u5F53\u524D\u7248\u672C\u53F7"
          },
          "download": {
            "title": "macOS\u4E0B\u8F7D\u5730\u5740",
            "description": "macOS\u5BA2\u6237\u7AEF\u4E0B\u8F7D\u94FE\u63A5"
          }
        },
        "android": {
          "version": {
            "title": "Android\u7248\u672C",
            "description": "Android\u5BA2\u6237\u7AEF\u5F53\u524D\u7248\u672C\u53F7"
          },
          "download": {
            "title": "Android\u4E0B\u8F7D\u5730\u5740",
            "description": "Android\u5BA2\u6237\u7AEF\u4E0B\u8F7D\u94FE\u63A5"
          }
        }
      },
      "common": {
        "saving": "\u4FDD\u5B58\u4E2D...",
        "save": "\u4FDD\u5B58",
        "save_success": "\u4FDD\u5B58\u6210\u529F",
        "save_error": "\u4FDD\u5B58\u5931\u8D25",
        "reset": "\u91CD\u7F6E",
        "placeholder": "\u8BF7\u8F93\u5165",
        "autoSaved": "\u5DF2\u81EA\u52A8\u4FDD\u5B58",
        "saved_at": "{{seconds}} \u79D2\u524D\u5DF2\u4FDD\u5B58",
        "invalidJson": "JSON \u683C\u5F0F\u9519\u8BEF"
      },
      "invite": {
        "title": "\u9080\u8BF7&\u4F63\u91D1\u8BBE\u7F6E",
        "description": "\u9080\u8BF7\u6CE8\u518C\u3001\u4F63\u91D1\u76F8\u5173\u8BBE\u7F6E\u3002",
        "invite_force": {
          "title": "\u5F00\u542F\u5F3A\u5236\u9080\u8BF7",
          "description": "\u5F00\u542F\u540E\u53EA\u6709\u88AB\u9080\u8BF7\u7684\u7528\u6237\u624D\u53EF\u4EE5\u8FDB\u884C\u6CE8\u518C\u3002"
        },
        "invite_commission": {
          "title": "\u9080\u8BF7\u4F63\u91D1\u767E\u5206\u6BD4",
          "description": "\u9ED8\u8BA4\u5168\u5C40\u7684\u4F63\u91D1\u5206\u914D\u6BD4\u4F8B\uFF0C\u4F60\u53EF\u4EE5\u5728\u7528\u6237\u7BA1\u7406\u5355\u72EC\u914D\u7F6E\u5355\u4E2A\u6BD4\u4F8B\u3002",
          "placeholder": "\u8BF7\u8F93\u5165\u4F63\u91D1\u767E\u5206\u6BD4"
        },
        "invite_gen_limit": {
          "title": "\u7528\u6237\u53EF\u521B\u5EFA\u9080\u8BF7\u7801\u4E0A\u9650",
          "description": "\u7528\u6237\u53EF\u521B\u5EFA\u9080\u8BF7\u7801\u4E0A\u9650",
          "placeholder": "\u8BF7\u8F93\u5165\u521B\u5EFA\u4E0A\u9650"
        },
        "invite_never_expire": {
          "title": "\u9080\u8BF7\u7801\u6C38\u4E0D\u5931\u6548",
          "description": "\u5F00\u542F\u540E\u9080\u8BF7\u7801\u88AB\u4F7F\u7528\u540E\u5C06\u4E0D\u4F1A\u5931\u6548\uFF0C\u5426\u5219\u4F7F\u7528\u8FC7\u540E\u5373\u5931\u6548\u3002"
        },
        "commission_first_time": {
          "title": "\u4F63\u91D1\u4EC5\u9996\u6B21\u53D1\u653E",
          "description": "\u5F00\u542F\u540E\u88AB\u9080\u8BF7\u4EBA\u9996\u6B21\u652F\u4ED8\u65F6\u624D\u4F1A\u4EA7\u751F\u4F63\u91D1\uFF0C\u53EF\u4EE5\u5728\u7528\u6237\u7BA1\u7406\u5BF9\u7528\u6237\u8FDB\u884C\u5355\u72EC\u914D\u7F6E\u3002"
        },
        "commission_auto_check": {
          "title": "\u4F63\u91D1\u81EA\u52A8\u786E\u8BA4",
          "description": "\u5F00\u542F\u540E\u4F63\u91D1\u5C06\u4F1A\u5728\u8BA2\u5355\u5B8C\u62103\u65E5\u540E\u81EA\u52A8\u8FDB\u884C\u786E\u8BA4\u3002"
        },
        "commission_withdraw_limit": {
          "title": "\u63D0\u73B0\u5355\u7533\u8BF7\u95E8\u69DB(\u5143)",
          "description": "\u5C0F\u4E8E\u95E8\u69DB\u91D1\u989D\u7684\u63D0\u73B0\u5355\u5C06\u4E0D\u4F1A\u88AB\u63D0\u4EA4\u3002",
          "placeholder": "\u8BF7\u8F93\u5165\u63D0\u73B0\u95E8\u69DB"
        },
        "commission_withdraw_method": {
          "title": "\u63D0\u73B0\u65B9\u5F0F",
          "description": "\u53EF\u4EE5\u652F\u6301\u7684\u63D0\u73B0\u65B9\u5F0F\uFF0C\u591A\u4E2A\u7528\u9017\u53F7\u5206\u9694\u3002",
          "placeholder": "\u8BF7\u8F93\u5165\u63D0\u73B0\u65B9\u5F0F\uFF0C\u591A\u4E2A\u7528\u9017\u53F7\u5206\u9694"
        },
        "withdraw_close": {
          "title": "\u5173\u95ED\u63D0\u73B0",
          "description": "\u5173\u95ED\u540E\u5C06\u7981\u6B62\u7528\u6237\u7533\u8BF7\u63D0\u73B0\uFF0C\u4E14\u9080\u8BF7\u4F63\u91D1\u5C06\u4F1A\u76F4\u63A5\u8FDB\u5165\u7528\u6237\u4F59\u989D\u3002"
        },
        "commission_distribution": {
          "title": "\u4E09\u7EA7\u5206\u9500",
          "description": "\u5F00\u542F\u540E\u5C06\u4F63\u91D1\u5C06\u6309\u7167\u8BBE\u7F6E\u76843\u6210\u6BD4\u4F8B\u8FDB\u884C\u5206\u6210\uFF0C\u4E09\u6210\u6BD4\u4F8B\u5408\u8BA1\u8BF7\u4E0D\u8981\u5927\u4E8E100%\u3002",
          "l1": "\u4E00\u7EA7\u9080\u8BF7\u4EBA\u6BD4\u4F8B",
          "l2": "\u4E8C\u7EA7\u9080\u8BF7\u4EBA\u6BD4\u4F8B",
          "l3": "\u4E09\u7EA7\u9080\u8BF7\u4EBA\u6BD4\u4F8B",
          "placeholder": "\u8BF7\u8F93\u5165\u6BD4\u4F8B\uFF0C\u5982\uFF1A50"
        },
        "distribution_total": "\u5F53\u524D\u5206\u9500\u6BD4\u4F8B\u5408\u8BA1\uFF1A{{total}}%\uFF08\u4E0D\u5E94\u8D85\u8FC7 100%\uFF09",
        "saving": "\u4FDD\u5B58\u4E2D..."
      },
      "server": {
        "title": "\u8282\u70B9\u914D\u7F6E",
        "description": "\u914D\u7F6E\u8282\u70B9\u901A\u4FE1\u548C\u540C\u6B65\u8BBE\u7F6E\uFF0C\u5305\u62EC\u901A\u4FE1\u5BC6\u94A5\u3001\u8F6E\u8BE2\u95F4\u9694\u3001\u8D1F\u8F7D\u5747\u8861\u7B49\u9AD8\u7EA7\u9009\u9879\u3002",
        "server_token": {
          "title": "\u901A\u8BAF\u5BC6\u94A5",
          "description": "Fboard\u4E0E\u8282\u70B9\u901A\u8BAF\u7684\u5BC6\u94A5\uFF0C\u4EE5\u4FBF\u6570\u636E\u4E0D\u4F1A\u88AB\u4ED6\u4EBA\u83B7\u53D6\u3002",
          "placeholder": "\u8BF7\u8F93\u5165\u901A\u8BAF\u5BC6\u94A5",
          "generate_tooltip": "\u70B9\u51FB\u751F\u6210\u968F\u673A\u901A\u4FE1\u5BC6\u94A5"
        },
        "server_pull_interval": {
          "title": "\u8282\u70B9\u62C9\u53D6\u52A8\u4F5C\u8F6E\u8BE2\u95F4\u9694",
          "description": "\u8282\u70B9\u4ECE\u9762\u677F\u83B7\u53D6\u6570\u636E\u7684\u95F4\u9694\u9891\u7387\u3002",
          "placeholder": "\u8BF7\u8F93\u5165\u62C9\u53D6\u95F4\u9694"
        },
        "server_push_interval": {
          "title": "\u8282\u70B9\u63A8\u9001\u52A8\u4F5C\u8F6E\u8BE2\u95F4\u9694",
          "description": "\u8282\u70B9\u63A8\u9001\u6570\u636E\u5230\u9762\u677F\u7684\u95F4\u9694\u9891\u7387\u3002",
          "placeholder": "\u8BF7\u8F93\u5165\u63A8\u9001\u95F4\u9694"
        },
        "device_limit_mode": {
          "title": "\u8BBE\u5907\u9650\u5236\u6A21\u5F0F",
          "description": "\u5BBD\u677E\u6A21\u5F0F\u4E0B\uFF0C\u540C\u4E00IP\u5730\u5740\u4F7F\u7528\u591A\u4E2A\u8282\u70B9\u53EA\u7EDF\u8BA1\u4E3A\u4E00\u4E2A\u8BBE\u5907\u3002",
          "strict": "\u4E25\u683C\u6A21\u5F0F",
          "relaxed": "\u5BBD\u677E\u6A21\u5F0F",
          "placeholder": "\u8BF7\u9009\u62E9\u8BBE\u5907\u9650\u5236\u6A21\u5F0F"
        },
        "server_ws_enable": {
          "title": "\u542F\u7528 WebSocket \u901A\u4FE1",
          "description": "\u5F00\u542F\u540E\u8282\u70B9\u5C06\u901A\u8FC7 WebSocket \u4E0E\u9762\u677F\u8FDB\u884C\u5B9E\u65F6\u901A\u4FE1\uFF0C\u5EF6\u8FDF\u66F4\u4F4E\u3001\u63A8\u9001\u66F4\u53CA\u65F6\u3002",
          "supported_clients": "\u76EE\u524D\u652F\u6301 WebSocket \u901A\u4FE1\u7684\u8282\u70B9\u7AEF\uFF1AFboard-Node"
        },
        "server_ws_url": {
          "title": "WebSocket \u5730\u5740",
          "description": "\u8282\u70B9\u8FDE\u63A5\u9762\u677F\u7684 WebSocket \u5730\u5740\uFF0C\u7559\u7A7A\u5219\u81EA\u52A8\u4F7F\u7528\u7AD9\u70B9\u7F51\u5740\u3002",
          "placeholder": "\u7559\u7A7A\u5219\u4F7F\u7528\u7AD9\u70B9\u7F51\u5740"
        },
        "server_ws_log_enable": {
          "title": "WebSocket \u8C03\u8BD5\u65E5\u5FD7",
          "description": "\u5F00\u542F\u540E\u8BB0\u5F55\u8282\u70B9/\u673A\u5668\u8FDE\u63A5\u3001\u5168\u91CF\u540C\u6B65\u3001\u63A8\u9001\u7B49 [WS] \u65E5\u5FD7\u3002\u9ED8\u8BA4\u5173\u95ED\uFF0C\u907F\u514D\u8282\u70B9\u8F83\u591A\u65F6\u5237\u5C4F\uFF1B\u5F02\u5E38\u544A\u8B66\u59CB\u7EC8\u4F1A\u5199\u5165\u3002\u4FEE\u6539\u540E\u6570\u79D2\u5185\u751F\u6548\uFF0C\u65E0\u9700\u91CD\u542F\u3002"
        },
        "node_install_script_url": {
          "title": "\u8282\u70B9\u5B89\u88C5\u811A\u672C\u5730\u5740",
          "description": "\u81EA\u5B9A\u4E49\u8282\u70B9\u5B89\u88C5\u811A\u672C\u7684 URL \u5730\u5740\uFF0C\u7559\u7A7A\u5219\u4F7F\u7528\u9ED8\u8BA4\u7684 GitHub \u5730\u5740\u3002",
          "placeholder": "\u7559\u7A7A\u5219\u4F7F\u7528\u9ED8\u8BA4\u5730\u5740"
        },
        "utls_fingerprints": {
          "title": "uTLS \u6307\u7EB9\u5217\u8868",
          "description": "\u7F16\u8F91\u8282\u70B9\u65F6\u300CuTLS \u8BBE\u7F6E \u2192 \u6307\u7EB9\u300D\u7684\u5177\u4F53\u6307\u7EB9\u53EF\u9009\u9879\u3002\u652F\u6301 chrome / firefox / safari / ios / android / edge / qq \u7B49\uFF0C\u4E5F\u53EF\u6DFB\u52A0\u5BA2\u6237\u7AEF\u652F\u6301\u7684\u5176\u5B83\u6307\u7EB9\u540D\u3002random\uFF08\u8BA2\u9605\u65F6\u968F\u673A\uFF09\u4E0E randomized\uFF08\u5BA2\u6237\u7AEF\u5185\u968F\u673A\uFF09\u7531\u7CFB\u7EDF\u56FA\u5B9A\u63D0\u4F9B\uFF0C\u65E0\u9700\u5728\u6B64\u914D\u7F6E\u3002",
          "placeholder": "\u8F93\u5165\u6307\u7EB9\u540D\u79F0\u540E\u56DE\u8F66\u6DFB\u52A0\uFF0C\u5982 chrome"
        },
        "saving": "\u4FDD\u5B58\u4E2D...",
        "manage": {
          "title": "\u8282\u70B9\u7BA1\u7406",
          "description": "\u7BA1\u7406\u6240\u6709\u8282\u70B9\uFF0C\u5305\u62EC\u6DFB\u52A0\u3001\u5220\u9664\u3001\u7F16\u8F91\u7B49\u64CD\u4F5C\u3002"
        }
      },
      "subscribe_template": {
        "title": "\u8BA2\u9605\u6A21\u677F",
        "description": "\u914D\u7F6E\u5404\u4E2A\u5BA2\u6237\u7AEF\u7684\u8BA2\u9605\u6A21\u677F\uFF08\u6309\u9700\u52A0\u8F7D\uFF0C\u9700\u624B\u52A8\u4FDD\u5B58\uFF09",
        "unsaved_hint": "\u5F53\u524D\u6A21\u677F\u6709\u672A\u4FDD\u5B58\u7684\u4FEE\u6539\uFF0C\u8BF7\u70B9\u51FB\u4FDD\u5B58\u3002",
        "singbox": {
          "title": "Sing-box \u8BA2\u9605\u6A21\u677F",
          "description": "\u914D\u7F6E Sing-box \u7684\u8BA2\u9605\u6A21\u677F\u683C\u5F0F"
        },
        "clash": {
          "title": "Clash \u8BA2\u9605\u6A21\u677F",
          "description": "\u914D\u7F6E Clash \u7684\u8BA2\u9605\u6A21\u677F\u683C\u5F0F"
        },
        "clashmeta": {
          "title": "Clash Meta \u8BA2\u9605\u6A21\u677F",
          "description": "\u914D\u7F6E Clash Meta \u7684\u8BA2\u9605\u6A21\u677F\u683C\u5F0F"
        },
        "stash": {
          "title": "Stash \u8BA2\u9605\u6A21\u677F",
          "description": "\u914D\u7F6E Stash \u7684\u8BA2\u9605\u6A21\u677F\u683C\u5F0F"
        },
        "surge": {
          "title": "Surge \u914D\u7F6E\u6A21\u677F",
          "description": "\u914D\u7F6E Surge \u8BA2\u9605\u6A21\u677F\uFF0C\u652F\u6301 Surge \u914D\u7F6E\u6587\u4EF6\u683C\u5F0F"
        },
        "surfboard": {
          "title": "Surfboard \u914D\u7F6E\u6A21\u7248",
          "description": "\u914D\u989D Surfboard \u8BA2\u9605\u6A21\u7248"
        }
      },
      "email_template": {
        "title": "\u90AE\u4EF6\u6A21\u677F",
        "description": "\u81EA\u5B9A\u4E49\u7CFB\u7EDF\u53D1\u9001\u7684\u5404\u7C7B\u90AE\u4EF6\u5185\u5BB9\u6A21\u677F",
        "customized": "\u5DF2\u81EA\u5B9A\u4E49",
        "subject": "\u90AE\u4EF6\u4E3B\u9898",
        "subject_placeholder": "\u8F93\u5165\u90AE\u4EF6\u4E3B\u9898\uFF0C\u652F\u6301 {{name}} \u7B49\u5360\u4F4D\u7B26",
        "content": "\u6A21\u677F\u5185\u5BB9 (HTML)",
        "preview": "\u5B9E\u65F6\u9884\u89C8",
        "override_hint": "\u4FEE\u6539\u5E76\u4FDD\u5B58\u540E\u5C06\u8986\u76D6\u7CFB\u7EDF\u9ED8\u8BA4\u6A21\u677F\u3002\u70B9\u51FB\u300C\u6062\u590D\u9ED8\u8BA4\u300D\u53EF\u968F\u65F6\u8FD8\u539F\u4E3A\u5F53\u524D\u4E3B\u9898\u7684\u9ED8\u8BA4\u6A21\u677F\u3002",
        "placeholders": "\u53EF\u7528\u5360\u4F4D\u7B26",
        "var_name": "\u53D8\u91CF",
        "var_desc": "\u8BF4\u660E",
        "var_sample": "\u793A\u4F8B\u503C",
        "required": "\u5FC5\u586B",
        "insert": "\u63D2\u5165",
        "placeholder_hint": "* \u6807\u8BB0\u4E3A\u5FC5\u987B\u5305\u542B\u7684\u5360\u4F4D\u7B26\uFF0C\u70B9\u51FB\u53EF\u63D2\u5165\u5230\u5185\u5BB9\u672B\u5C3E",
        "click_to_insert": "\u70B9\u51FB\u63D2\u5165",
        "save": "\u4FDD\u5B58",
        "save_success": "\u6A21\u677F\u4FDD\u5B58\u6210\u529F",
        "save_before_test": "\u8BF7\u5148\u4FDD\u5B58\u4FEE\u6539\u540E\u518D\u53D1\u9001\u6D4B\u8BD5",
        "send_test": "\u53D1\u9001\u6D4B\u8BD5",
        "test_dialog_title": "\u53D1\u9001\u6D4B\u8BD5\u90AE\u4EF6",
        "test_dialog_description": "\u8F93\u5165\u6536\u4EF6\u90AE\u7BB1\uFF0C\u7559\u7A7A\u5C06\u53D1\u9001\u5230\u5F53\u524D\u7BA1\u7406\u5458\u90AE\u7BB1",
        "test_email_placeholder": "\u6536\u4EF6\u90AE\u7BB1\uFF08\u7559\u7A7A\u4F7F\u7528\u5F53\u524D\u8D26\u53F7\uFF09",
        "sending": "\u53D1\u9001\u4E2D...",
        "test_success": "\u6D4B\u8BD5\u90AE\u4EF6\u5DF2\u53D1\u9001",
        "reset": "\u6062\u590D\u9ED8\u8BA4",
        "reset_title": "\u6062\u590D\u9ED8\u8BA4\u6A21\u677F",
        "reset_description": "\u786E\u5B9A\u8981\u6062\u590D\u6B64\u6A21\u677F\u4E3A\u9ED8\u8BA4\u5185\u5BB9\u5417\uFF1F\u81EA\u5B9A\u4E49\u7684\u5185\u5BB9\u5C06\u88AB\u5220\u9664\u3002",
        "reset_confirm": "\u786E\u5B9A\u6062\u590D",
        "reset_success": "\u5DF2\u6062\u590D\u9ED8\u8BA4\u6A21\u677F",
        "unsaved": "\u6709\u672A\u4FDD\u5B58\u7684\u4FEE\u6539",
        "discard_title": "\u672A\u4FDD\u5B58\u7684\u4FEE\u6539",
        "discard_description": "\u5F53\u524D\u6A21\u677F\u6709\u672A\u4FDD\u5B58\u7684\u4FEE\u6539\uFF0C\u5207\u6362\u6807\u7B7E\u9875\u5C06\u4E22\u5931\u8FD9\u4E9B\u4FEE\u6539\u3002",
        "discard_confirm": "\u4E22\u5F03\u4FEE\u6539",
        "cancel": "\u53D6\u6D88"
      }
    },
    "notice": {
      "title": "\u516C\u544A\u7BA1\u7406",
      "description": "\u5728\u8FD9\u91CC\u53EF\u4EE5\u914D\u7F6E\u516C\u544A\uFF0C\u5305\u62EC\u6DFB\u52A0\u3001\u5220\u9664\u3001\u7F16\u8F91\u7B49\u64CD\u4F5C\u3002",
      "table": {
        "columns": {
          "id": "ID",
          "show": "\u663E\u793A\u72B6\u6001",
          "title": "\u6807\u9898",
          "actions": "\u64CD\u4F5C"
        },
        "toolbar": {
          "search": "\u641C\u7D22\u516C\u544A\u6807\u9898...",
          "reset": "\u91CD\u7F6E",
          "sort": {
            "edit": "\u7F16\u8F91\u6392\u5E8F",
            "save": "\u4FDD\u5B58\u6392\u5E8F"
          }
        },
        "actions": {
          "edit": "\u7F16\u8F91",
          "delete": {
            "title": "\u5220\u9664\u786E\u8BA4",
            "description": "\u786E\u5B9A\u8981\u5220\u9664\u8BE5\u6761\u516C\u544A\u5417\uFF1F\u6B64\u64CD\u4F5C\u65E0\u6CD5\u64A4\u9500\u3002",
            "success": "\u5220\u9664\u6210\u529F"
          }
        }
      },
      "form": {
        "add": {
          "title": "\u6DFB\u52A0\u516C\u544A",
          "button": "\u6DFB\u52A0\u516C\u544A"
        },
        "edit": {
          "title": "\u7F16\u8F91\u516C\u544A"
        },
        "fields": {
          "title": {
            "label": "\u6807\u9898",
            "placeholder": "\u8BF7\u8F93\u5165\u516C\u544A\u6807\u9898"
          },
          "content": {
            "label": "\u516C\u544A\u5185\u5BB9"
          },
          "img_url": {
            "label": "\u516C\u544A\u80CC\u666F",
            "placeholder": "\u8BF7\u8F93\u5165\u516C\u544A\u80CC\u666F\u56FE\u7247URL"
          },
          "show": {
            "label": "\u663E\u793A"
          },
          "tags": {
            "label": "\u8282\u70B9\u6807\u7B7E",
            "placeholder": "\u8F93\u5165\u540E\u56DE\u8F66\u6DFB\u52A0\u6807\u7B7E"
          }
        },
        "buttons": {
          "cancel": "\u53D6\u6D88",
          "submit": "\u63D0\u4EA4",
          "success": "\u63D0\u4EA4\u6210\u529F"
        }
      },
      "messages": {
        "loadError": "\u52A0\u8F7D\u516C\u544A\u5931\u8D25"
      }
    },
    "group": {
      "title": "\u6743\u9650\u7EC4\u7BA1\u7406",
      "description": "\u7BA1\u7406\u6240\u6709\u6743\u9650\u7EC4\uFF0C\u5305\u62EC\u6DFB\u52A0\u3001\u5220\u9664\u3001\u7F16\u8F91\u7B49\u64CD\u4F5C\u3002",
      "columns": {
        "id": "\u7EC4ID",
        "name": "\u7EC4\u540D\u79F0",
        "usersCount": "\u7528\u6237\u6570\u91CF",
        "serverCount": "\u8282\u70B9\u6570\u91CF",
        "actions": "\u64CD\u4F5C"
      },
      "form": {
        "add": "\u6DFB\u52A0\u6743\u9650\u7EC4",
        "edit": "\u7F16\u8F91\u6743\u9650\u7EC4",
        "create": "\u521B\u5EFA\u6743\u9650\u7EC4",
        "update": "\u66F4\u65B0",
        "name": "\u7EC4\u540D\u79F0",
        "namePlaceholder": "\u8BF7\u8F93\u5165\u6743\u9650\u7EC4\u540D\u79F0",
        "nameDescription": "\u6743\u9650\u7EC4\u540D\u79F0\u7528\u4E8E\u6807\u8BC6\u4E0D\u540C\u7684\u7528\u6237\u7EC4\uFF0C\u5EFA\u8BAE\u4F7F\u7528\u6709\u610F\u4E49\u7684\u540D\u79F0\u3002",
        "cancel": "\u53D6\u6D88",
        "editDescription": "\u4FEE\u6539\u6743\u9650\u7EC4\u4FE1\u606F\uFF0C\u66F4\u65B0\u540E\u4F1A\u7ACB\u5373\u751F\u6548\u3002",
        "createDescription": "\u521B\u5EFA\u65B0\u7684\u6743\u9650\u7EC4\uFF0C\u53EF\u4EE5\u4E3A\u4E0D\u540C\u7684\u7528\u6237\u5206\u914D\u4E0D\u540C\u7684\u6743\u9650\u3002"
      },
      "toolbar": {
        "searchPlaceholder": "\u641C\u7D22\u6743\u9650\u7EC4...",
        "reset": "\u91CD\u7F6E"
      },
      "messages": {
        "deleteConfirm": "\u786E\u8BA4\u5220\u9664",
        "deleteDescription": "\u6B64\u64CD\u4F5C\u5C06\u6C38\u4E45\u5220\u9664\u8BE5\u6743\u9650\u7EC4\uFF0C\u5220\u9664\u540E\u65E0\u6CD5\u6062\u590D\u3002\u786E\u5B9A\u8981\u7EE7\u7EED\u5417\uFF1F",
        "deleteButton": "\u5220\u9664",
        "createSuccess": "\u521B\u5EFA\u6210\u529F",
        "updateSuccess": "\u66F4\u65B0\u6210\u529F",
        "nameValidation": {
          "min": "\u7EC4\u540D\u81F3\u5C11\u9700\u89812\u4E2A\u5B57\u7B26",
          "max": "\u7EC4\u540D\u4E0D\u80FD\u8D85\u8FC750\u4E2A\u5B57\u7B26",
          "pattern": "\u7EC4\u540D\u53EA\u80FD\u5305\u542B\u5B57\u6BCD\u3001\u6570\u5B57\u3001\u4E2D\u6587\u3001\u4E0B\u5212\u7EBF\u548C\u8FDE\u5B57\u7B26"
        }
      }
    },
    "auth": {
      "signIn": {
        "title": "\u767B\u5F55",
        "description": "\u8BF7\u8F93\u5165\u60A8\u7684\u90AE\u7BB1\u548C\u5BC6\u7801\u767B\u5F55\u7CFB\u7EDF",
        "email": "\u90AE\u7BB1\u5730\u5740",
        "emailPlaceholder": "name@example.com",
        "password": "\u5BC6\u7801",
        "passwordPlaceholder": "\u8BF7\u8F93\u5165\u5BC6\u7801",
        "forgotPassword": "\u5FD8\u8BB0\u5BC6\u7801\uFF1F",
        "submit": "\u767B\u5F55",
        "rememberMe": "\u8BB0\u4F4F\u6211",
        "resetPassword": {
          "title": "\u91CD\u7F6E\u5BC6\u7801",
          "description": "\u5728\u7AD9\u70B9\u76EE\u5F55\u4E0B\u6267\u884C\u4EE5\u4E0B\u547D\u4EE4\u627E\u56DE\u5BC6\u7801",
          "command": "php artisan reset:password \u7BA1\u7406\u5458\u90AE\u7BB1"
        },
        "validation": {
          "emailRequired": "\u8BF7\u8F93\u5165\u90AE\u7BB1\u5730\u5740",
          "emailInvalid": "\u90AE\u7BB1\u5730\u5740\u683C\u5F0F\u4E0D\u6B63\u786E",
          "passwordRequired": "\u8BF7\u8F93\u5165\u5BC6\u7801",
          "passwordLength": "\u5BC6\u7801\u957F\u5EA6\u81F3\u5C11\u4E3A7\u4E2A\u5B57\u7B26"
        }
      }
    },
    "traffic": {
      "trafficRecord": {
        "title": "\u6D41\u91CF\u4F7F\u7528\u8BB0\u5F55",
        "time": "\u65F6\u95F4",
        "upload": "\u4E0A\u884C\u6D41\u91CF",
        "download": "\u4E0B\u884C\u6D41\u91CF",
        "rate": "\u500D\u7387",
        "total": "\u603B\u8BA1",
        "noRecords": "\u6682\u65E0\u8BB0\u5F55",
        "perPage": "\u6BCF\u9875\u663E\u793A",
        "records": "\u6761\u8BB0\u5F55",
        "page": "\u7B2C {{current}} / {{total}} \u9875",
        "multiplier": "{{value}}x"
      }
    },
    "payment": {
      "title": "\u652F\u4ED8\u914D\u7F6E",
      "description": "\u5728\u8FD9\u91CC\u53EF\u4EE5\u914D\u7F6E\u652F\u4ED8\u65B9\u5F0F\uFF0C\u5305\u62EC\u652F\u4ED8\u5B9D\u3001\u5FAE\u4FE1\u7B49\u3002",
      "table": {
        "columns": {
          "id": "ID",
          "enable": "\u542F\u7528",
          "name": "\u663E\u793A\u540D\u79F0",
          "payment": "\u652F\u4ED8\u63A5\u53E3",
          "notify_url": "\u901A\u77E5\u5730\u5740",
          "notify_url_tooltip": "\u652F\u4ED8\u7F51\u5173\u5C06\u4F1A\u628A\u6570\u636E\u901A\u77E5\u5230\u672C\u5730\u5740\uFF0C\u8BF7\u901A\u8FC7\u9632\u706B\u5899\u653E\u884C\u672C\u5730\u5740\u3002",
          "actions": "\u64CD\u4F5C"
        },
        "actions": {
          "edit": "\u7F16\u8F91",
          "copy": "\u590D\u5236",
          "copy_success": "\u590D\u5236\u6210\u529F",
          "delete": {
            "title": "\u5220\u9664\u786E\u8BA4",
            "description": "\u786E\u5B9A\u8981\u5220\u9664\u8BE5\u652F\u4ED8\u65B9\u5F0F\u5417\uFF1F\u6B64\u64CD\u4F5C\u65E0\u6CD5\u64A4\u9500\u3002",
            "success": "\u5220\u9664\u6210\u529F"
          }
        },
        "toolbar": {
          "search": "\u641C\u7D22\u652F\u4ED8\u65B9\u5F0F...",
          "reset": "\u91CD\u7F6E",
          "sort": {
            "hint": "\u62D6\u62FD\u652F\u4ED8\u65B9\u5F0F\u8FDB\u884C\u6392\u5E8F\uFF0C\u5B8C\u6210\u540E\u70B9\u51FB\u4FDD\u5B58",
            "save": "\u4FDD\u5B58\u6392\u5E8F",
            "edit": "\u7F16\u8F91\u6392\u5E8F"
          }
        }
      },
      "form": {
        "add": {
          "button": "\u6DFB\u52A0\u652F\u4ED8\u65B9\u5F0F",
          "title": "\u6DFB\u52A0\u652F\u4ED8\u65B9\u5F0F"
        },
        "edit": {
          "title": "\u7F16\u8F91\u652F\u4ED8\u65B9\u5F0F"
        },
        "fields": {
          "name": {
            "label": "\u663E\u793A\u540D\u79F0",
            "placeholder": "\u8BF7\u8F93\u5165\u652F\u4ED8\u540D\u79F0",
            "description": "\u7528\u4E8E\u524D\u7AEF\u663E\u793A"
          },
          "icon": {
            "label": "\u56FE\u6807URL",
            "placeholder": "https://example.com/icon.svg",
            "description": "\u7528\u4E8E\u524D\u7AEF\u663E\u793A\u7684\u56FE\u6807\u5730\u5740"
          },
          "notify_domain": {
            "label": "\u901A\u77E5\u57DF\u540D",
            "placeholder": "https://example.com",
            "description": "\u7F51\u5173\u901A\u77E5\u5C06\u53D1\u9001\u5230\u8BE5\u57DF\u540D"
          },
          "handling_fee_percent": {
            "label": "\u767E\u5206\u6BD4\u624B\u7EED\u8D39(%)",
            "placeholder": "0-100"
          },
          "handling_fee_fixed": {
            "label": "\u56FA\u5B9A\u624B\u7EED\u8D39",
            "placeholder": "0"
          },
          "payment": {
            "label": "\u652F\u4ED8\u63A5\u53E3",
            "placeholder": "\u8BF7\u9009\u62E9\u652F\u4ED8\u63A5\u53E3",
            "description": "\u9009\u62E9\u8981\u4F7F\u7528\u7684\u652F\u4ED8\u63A5\u53E3"
          }
        },
        "validation": {
          "name": {
            "min": "\u540D\u79F0\u81F3\u5C11\u9700\u89812\u4E2A\u5B57\u7B26",
            "max": "\u540D\u79F0\u4E0D\u80FD\u8D85\u8FC730\u4E2A\u5B57\u7B26"
          },
          "notify_domain": {
            "url": "\u8BF7\u8F93\u5165\u6709\u6548\u7684URL"
          },
          "payment": {
            "required": "\u8BF7\u9009\u62E9\u652F\u4ED8\u63A5\u53E3"
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
          "noConfig": "\u8BE5\u652F\u4ED8\u65B9\u5F0F\u6682\u65E0\u914D\u7F6E\u9879"
        }
      }
    },
    "server": {
      "manage": {
        "title": "\u8282\u70B9\u7BA1\u7406",
        "description": "\u7BA1\u7406\u6240\u6709\u8282\u70B9\uFF0C\u5305\u62EC\u6DFB\u52A0\u3001\u5220\u9664\u3001\u7F16\u8F91\u7B49\u64CD\u4F5C\u3002",
        "filtered_by_server": "\u5F53\u524D\u6B63\u5728\u67E5\u770B\u670D\u52A1\u5668 {{server}} (SID:{{id}}) \u4E0B\u7684\u8282\u70B9",
        "filtered_by_server_description": "\u5728\u8FD9\u91CC\u65B0\u589E\u8282\u70B9\u65F6\uFF0C\u53EF\u4EE5\u76F4\u63A5\u590D\u7528\u5F53\u524D\u670D\u52A1\u5668\u4F5C\u4E3A\u90E8\u7F72\u76EE\u6807\u3002",
        "add_node_to_server": "\u65B0\u589E\u8282\u70B9\u5230\u6B64\u670D\u52A1\u5668",
        "clear_server_filter": "\u6E05\u9664\u670D\u52A1\u5668\u7B5B\u9009"
      },
      "columns": {
        "sort": "\u6392\u5E8F",
        "nodeId": "\u8282\u70B9ID",
        "show": "\u663E\u9690",
        "node": "\u8282\u70B9",
        "address": "\u5730\u5740",
        "onlineUsers": {
          "title": "\u5728\u7EBF\u4EBA\u6570",
          "tooltip": "\u5728\u7EBF\u4EBA\u6570\u6839\u636E\u670D\u52A1\u7AEF\u4E0A\u62A5\u9891\u7387\u800C\u5B9A",
          "sort_tip": "\u6309\u5728\u7EBF\u4EBA\u6570\u6392\u5E8F"
        },
        "rate": {
          "title": "\u500D\u7387",
          "tooltip": "\u6D41\u91CF\u6263\u8D39\u500D\u7387"
        },
        "traffic": {
          "title": "\u6D41\u91CF\u4F7F\u7528",
          "tooltip": "\u8282\u70B9\u6D41\u91CF\u4F7F\u7528\u60C5\u51B5\uFF0C\u663E\u793A\u5DF2\u7528\u6D41\u91CF\u548C\u9650\u5236",
          "total": "\u603B\u6D41\u91CF",
          "used": "\u5DF2\u7528",
          "percentage": "\u4F7F\u7528\u7387"
        },
        "groups": {
          "title": "\u6743\u9650\u7EC4",
          "tooltip": "\u53EF\u8BA2\u9605\u5230\u8BE5\u8282\u70B9\u7684\u6743\u9650\u7EC4",
          "empty": "--"
        },
        "loadStatus": {
          "title": "\u8D1F\u8F7D\u72B6\u6001",
          "tooltip": "\u670D\u52A1\u5668\u8D44\u6E90\u4F7F\u7528\u60C5\u51B5",
          "noData": "\u6682\u65E0\u6570\u636E",
          "details": "\u7CFB\u7EDF\u8D1F\u8F7D\u8BE6\u60C5",
          "cpu": "CPU \u4F7F\u7528\u7387",
          "memory": "\u5185\u5B58\u4F7F\u7528",
          "swap": "\u4EA4\u6362\u533A",
          "disk": "\u78C1\u76D8\u4F7F\u7528",
          "lastUpdate": "\u6700\u540E\u66F4\u65B0",
          "metrics": {
            "title": "\u8FD0\u884C\u6307\u6807",
            "uptime": "\u8FD0\u884C\u65F6\u957F",
            "conns": "\u5B9E\u65F6/\u603B\u8FDE\u63A5",
            "speed": "\u5B9E\u65F6\u901F\u7387",
            "api": "API \u72B6\u6001",
            "kernel": "\u5185\u6838\u72B6\u6001",
            "gc": "GC \u6682\u505C",
            "limit": "\u9650\u901F\u7528\u6237",
            "ws": "WebSocket",
            "goroutines": "\u5E76\u53D1\u534F\u7A0B",
            "load": "\u7CFB\u7EDF\u8D1F\u8F7D",
            "users": "\u5728\u7EBF\u7528\u6237"
          }
        },
        "version": "\u7248\u672C",
        "customId": "\u81EA\u5B9A\u4E49ID",
        "originalId": "\u539F\u59CBID",
        "type": "\u7C7B\u578B",
        "actions": "\u64CD\u4F5C",
        "copyAddress": "\u590D\u5236\u8FDE\u63A5\u5730\u5740",
        "internalPort": "\u5185\u90E8\u7AEF\u53E3",
        "deployment": {
          "title": "\u90E8\u7F72\u65B9\u5F0F",
          "tooltip": "\u67E5\u770B\u8282\u70B9\u662F\u72EC\u7ACB\u90E8\u7F72\uFF0C\u8FD8\u662F\u7531\u67D0\u53F0\u670D\u52A1\u5668\u6258\u7BA1\uFF0C\u5E76\u53EF\u76F4\u63A5\u5728\u5217\u8868\u4E2D\u8C03\u6574\u3002",
          "standalone": "\u72EC\u7ACB\u90E8\u7F72",
          "standalone_row_hint": "\u672A\u7ED1\u5B9A\u670D\u52A1\u5668",
          "standalone_description": "\u8BE5\u8282\u70B9\u4E0D\u4F9D\u8D56\u670D\u52A1\u5668\u6258\u7BA1\uFF0C\u9002\u7528\u4E8E\u5355\u8282\u70B9\u72EC\u7ACB\u90E8\u7F72\u3002",
          "online": "\u5728\u7EBF",
          "offline": "\u79BB\u7EBF",
          "inactive": "\u670D\u52A1\u5668\u672A\u6FC0\u6D3B",
          "disabled": "\u8282\u70B9\u505C\u7528",
          "enabled": "\u5728\u670D\u52A1\u5668\u4E0A\u542F\u7528",
          "enabled_description": "\u4EC5\u5DF2\u542F\u7528\u7684\u8282\u70B9\u4F1A\u7531\u6240\u9009\u670D\u52A1\u5668\u62C9\u8D77\u5E76\u540C\u6B65\u3002",
          "enabled_standalone_description": "\u72EC\u7ACB\u90E8\u7F72\u8282\u70B9\u65E0\u9700\u8BBE\u7F6E\u670D\u52A1\u5668\u542F\u7528\u72B6\u6001\u3002",
          "bind_success": "\u5DF2\u6258\u7BA1\u5230 {{server}}",
          "standalone_success": "\u5DF2\u5207\u6362\u4E3A\u72EC\u7ACB\u90E8\u7F72",
          "update_success": "\u90E8\u7F72\u72B6\u6001\u5DF2\u66F4\u65B0",
          "update_error": "\u90E8\u7F72\u72B6\u6001\u66F4\u65B0\u5931\u8D25"
        },
        "status": {
          "0": "\u672A\u8FD0\u884C",
          "1": "\u65E0\u4EBA\u4F7F\u7528\u6216\u5F02\u5E38",
          "2": "\u8FD0\u884C\u6B63\u5E38"
        },
        "childNode": "\u5B50\u8282\u70B9",
        "actions_dropdown": {
          "edit": "\u7F16\u8F91",
          "copy": "\u590D\u5236",
          "reset_traffic": {
            "title": "\u786E\u8BA4\u91CD\u7F6E\u6D41\u91CF",
            "description": "\u6B64\u64CD\u4F5C\u5C06\u6E05\u96F6\u8BE5\u8282\u70B9\u7684\u4E0A\u4F20\u548C\u4E0B\u8F7D\u6D41\u91CF\uFF0C\u5E76\u89E3\u9664\u7981\u7528\u72B6\u6001\u3002\u786E\u5B9A\u8981\u7EE7\u7EED\u5417\uFF1F",
            "confirm": "\u91CD\u7F6E\u6D41\u91CF"
          },
          "reset_traffic_success": "\u6D41\u91CF\u91CD\u7F6E\u6210\u529F",
          "delete": {
            "title": "\u786E\u8BA4\u5220\u9664",
            "description": "\u6B64\u64CD\u4F5C\u5C06\u6C38\u4E45\u5220\u9664\u8BE5\u8282\u70B9\uFF0C\u5220\u9664\u540E\u65E0\u6CD5\u6062\u590D\u3002\u786E\u5B9A\u8981\u7EE7\u7EED\u5417\uFF1F",
            "confirm": "\u5220\u9664"
          },
          "copy_success": "\u590D\u5236\u6210\u529F",
          "delete_success": "\u5220\u9664\u6210\u529F"
        }
      },
      "toolbar": {
        "search": "\u641C\u7D22\u8282\u70B9...",
        "type": "\u7C7B\u578B",
        "status": "\u8FD0\u884C\u72B6\u6001",
        "server": "\u670D\u52A1\u5668",
        "server_search": "\u641C\u7D22\u670D\u52A1\u5668...",
        "server_empty": "\u672A\u627E\u5230\u670D\u52A1\u5668",
        "reset": "\u91CD\u7F6E",
        "actions": "\u64CD\u4F5C",
        "sort": {
          "tip": "\u62D6\u62FD\u8282\u70B9\u8FDB\u884C\u6392\u5E8F\uFF0C\u5B8C\u6210\u540E\u70B9\u51FB\u4FDD\u5B58",
          "edit": "\u7F16\u8F91\u6392\u5E8F",
          "save": "\u4FDD\u5B58\u6392\u5E8F",
          "success": "\u6392\u5E8F\u4FDD\u5B58\u6210\u529F"
        },
        "batch_delete": {
          "menu": "\u5220\u9664\u8282\u70B9",
          "button": "\u5220\u9664 {{count}} \u9879",
          "title": "\u786E\u8BA4\u6279\u91CF\u5220\u9664",
          "description": "\u786E\u5B9A\u8981\u5220\u9664\u9009\u4E2D\u7684 {{count}} \u4E2A\u8282\u70B9\u5417\uFF1F\u6B64\u64CD\u4F5C\u4E0D\u53EF\u6062\u590D\u3002",
          "confirm": "\u786E\u8BA4\u5220\u9664"
        },
        "batch_delete_success": "\u6210\u529F\u5220\u9664 {{count}} \u4E2A\u8282\u70B9",
        "batch_delete_error": "\u6279\u91CF\u5220\u9664\u5931\u8D25",
        "batch_show": {
          "menu": "\u663E\u793A\u8282\u70B9"
        },
        "batch_show_success": "\u6210\u529F\u663E\u793A {{count}} \u4E2A\u8282\u70B9",
        "batch_show_error": "\u6279\u91CF\u663E\u793A\u5931\u8D25",
        "batch_hide": {
          "menu": "\u9690\u85CF\u8282\u70B9"
        },
        "batch_hide_success": "\u6210\u529F\u9690\u85CF {{count}} \u4E2A\u8282\u70B9",
        "batch_hide_error": "\u6279\u91CF\u9690\u85CF\u5931\u8D25",
        "batch_enable": {
          "menu": "\u542F\u7528\u8282\u70B9"
        },
        "batch_enable_success": "\u6210\u529F\u542F\u7528 {{count}} \u4E2A\u8282\u70B9",
        "batch_enable_error": "\u6279\u91CF\u542F\u7528\u5931\u8D25",
        "batch_disable": {
          "menu": "\u7981\u7528\u8282\u70B9"
        },
        "batch_disable_success": "\u6210\u529F\u7981\u7528 {{count}} \u4E2A\u8282\u70B9",
        "batch_disable_error": "\u6279\u91CF\u7981\u7528\u5931\u8D25",
        "batch_reset_traffic": {
          "menu": "\u91CD\u7F6E\u6D41\u91CF",
          "button": "\u91CD\u7F6E {{count}} \u9879\u6D41\u91CF",
          "title": "\u786E\u8BA4\u6279\u91CF\u91CD\u7F6E\u6D41\u91CF",
          "description": "\u786E\u5B9A\u8981\u91CD\u7F6E\u9009\u4E2D\u7684 {{count}} \u4E2A\u8282\u70B9\u7684\u6D41\u91CF\u5417\uFF1F\u6B64\u64CD\u4F5C\u5C06\u6E05\u96F6\u6D41\u91CF\u5E76\u89E3\u9664\u7981\u7528\u72B6\u6001\u3002",
          "confirm": "\u786E\u8BA4\u91CD\u7F6E"
        },
        "batch_reset_traffic_success": "\u6210\u529F\u91CD\u7F6E {{count}} \u4E2A\u8282\u70B9\u7684\u6D41\u91CF",
        "batch_reset_traffic_error": "\u6279\u91CF\u91CD\u7F6E\u6D41\u91CF\u5931\u8D25",
        "batch_replace": {
          "selected": "\u5DF2\u9009\u62E9 {{count}} \u4E2A\u8282\u70B9",
          "menu": "\u6279\u91CF\u66FF\u6362",
          "clear": "\u6E05\u9664\u9009\u62E9",
          "title": "\u6279\u91CF\u66FF\u6362\u8282\u70B9\u5B57\u6BB5",
          "field": "\u5B57\u6BB5",
          "search": "\u641C\u7D22\u503C",
          "search_placeholder": "\u8F93\u5165\u8981\u641C\u7D22\u7684\u5B57\u7B26\u4E32",
          "replace": "\u66FF\u6362\u503C",
          "replace_placeholder": "\u8F93\u5165\u8981\u66FF\u6362\u6210\u7684\u5B57\u7B26\u4E32",
          "confirm": "\u66FF\u6362",
          "search_required": "\u8BF7\u8F93\u5165\u641C\u7D22\u503C",
          "success": "\u6210\u529F\u66FF\u6362 {{count}} \u4E2A\u8282\u70B9",
          "fields": {
            "name": "\u8282\u70B9\u540D\u79F0",
            "host": "\u4E3B\u673A\u5730\u5740",
            "port": "\u7AEF\u53E3",
            "code": "\u8282\u70B9\u6807\u8BC6",
            "group_ids": "\u6743\u9650\u7EC4",
            "route_ids": "\u8DEF\u7531\u89C4\u5219",
            "tags": "\u6807\u7B7E",
            "protocol_settings": "\u534F\u8BAE\u8BBE\u7F6E",
            "custom_outbounds": "\u81EA\u5B9A\u4E49\u51FA\u7AD9",
            "custom_routes": "\u81EA\u5B9A\u4E49\u8DEF\u7531",
            "cert_config": "\u8BC1\u4E66\u914D\u7F6E",
            "rate_time_ranges": "\u500D\u7387\u65F6\u95F4\u6BB5"
          }
        },
        "filteringByMachine": "\u6B63\u5728\u7B5B\u9009\u8BE5\u673A\u5668\u7684\u8282\u70B9\uFF1A",
        "virtualNode": "\u865A\u62DF\u8282\u70B9"
      },
      "form": {
        "add_node": "\u6DFB\u52A0\u8282\u70B9",
        "edit_node": "\u7F16\u8F91\u8282\u70B9",
        "new_node": "\u65B0\u5EFA\u8282\u70B9",
        "type": {
          "placeholder": "\u9009\u62E9\u534F\u8BAE\u7C7B\u578B",
          "select_prompt": "\u8BF7\u5148\u9009\u62E9\u534F\u8BAE\u7C7B\u578B",
          "select_error": "\u8BF7\u5148\u9009\u62E9\u534F\u8BAE\u7C7B\u578B",
          "configHint": "\u9009\u62E9\u534F\u8BAE\u7C7B\u578B\u540E\u53EF\u914D\u7F6E"
        },
        "name": {
          "label": "\u8282\u70B9\u540D\u79F0",
          "placeholder": "\u8BF7\u8F93\u5165\u8282\u70B9\u540D\u79F0",
          "error": "\u8BF7\u8F93\u5165\u6709\u6548\u7684\u8282\u70B9\u540D\u79F0"
        },
        "rate": {
          "label": "\u57FA\u7840\u500D\u7387",
          "hint": "\u6D41\u91CF\u8BA1\u8D39\u500D\u7387\uFF0C1 \u8868\u793A\u6309\u5B9E\u9645\u6D41\u91CF\u8BA1\u8D39",
          "error": "\u57FA\u7840\u500D\u7387\u4E0D\u80FD\u4E3A\u7A7A",
          "error_numeric": "\u57FA\u7840\u500D\u7387\u5FC5\u987B\u662F\u6570\u5B57",
          "error_gte_zero": "\u57FA\u7840\u500D\u7387\u5FC5\u987B\u5927\u4E8E\u6216\u7B49\u4E8E0",
          "child_node_tooltip": "\u5B50\u8282\u70B9\u7684\u57FA\u7840\u500D\u7387\u7EE7\u627F\u81EA\u7236\u8282\u70B9\uFF0C\u65E0\u6CD5\u5355\u72EC\u8BBE\u7F6E",
          "child_node_note": "\u5B50\u8282\u70B9\u500D\u7387\u7EE7\u627F\u81EA\u7236\u8282\u70B9"
        },
        "traffic_limit": {
          "label": "\u6D41\u91CF\u9650\u5236",
          "placeholder": "0 \u8868\u793A\u4E0D\u9650\u5236",
          "hint": "\u8BBE\u7F6E\u8282\u70B9\u6D41\u91CF\u4E0A\u9650\uFF08\u5355\u4F4D\uFF1AGB\uFF09\uFF0C0 \u8868\u793A\u4E0D\u9650\u5236",
          "error_numeric": "\u6D41\u91CF\u9650\u5236\u5FC5\u987B\u662F\u6570\u5B57",
          "error_gte_zero": "\u6D41\u91CF\u9650\u5236\u5FC5\u987B\u5927\u4E8E\u6216\u7B49\u4E8E0"
        },
        "protocolSection": "\u534F\u8BAE\u914D\u7F6E",
        "traffic_limit_unit": "GB, 0=\u4E0D\u9650\u5236",
        "banned": {
          "label": "\u7981\u7528\u8282\u70B9",
          "description": "\u7981\u7528\u540E\u8282\u70B9\u5C06\u4E0D\u53EF\u7528"
        },
        "dynamic_rate": {
          "section_title": "\u52A8\u6001\u500D\u7387\u914D\u7F6E",
          "enable_label": "\u542F\u7528\u52A8\u6001\u500D\u7387",
          "enable_description": "\u6839\u636E\u65F6\u95F4\u6BB5\u8BBE\u7F6E\u4E0D\u540C\u7684\u500D\u7387\u4E58\u6570",
          "rules_label": "\u65F6\u95F4\u6BB5\u89C4\u5219",
          "add_rule": "\u6DFB\u52A0\u89C4\u5219",
          "rule_title": "\u89C4\u5219 {{index}}",
          "start_time": "\u5F00\u59CB\u65F6\u95F4",
          "end_time": "\u7ED3\u675F\u65F6\u95F4",
          "multiplier": "\u500D\u7387\u4E58\u6570",
          "no_rules": "\u6682\u65E0\u89C4\u5219\uFF0C\u70B9\u51FB\u4E0A\u65B9\u6309\u94AE\u6DFB\u52A0",
          "start_time_error": "\u5F00\u59CB\u65F6\u95F4\u4E0D\u80FD\u4E3A\u7A7A",
          "end_time_error": "\u7ED3\u675F\u65F6\u95F4\u4E0D\u80FD\u4E3A\u7A7A",
          "multiplier_error": "\u500D\u7387\u4E58\u6570\u4E0D\u80FD\u4E3A\u7A7A",
          "multiplier_error_numeric": "\u500D\u7387\u4E58\u6570\u5FC5\u987B\u662F\u6570\u5B57",
          "multiplier_error_gte_zero": "\u500D\u7387\u4E58\u6570\u5FC5\u987B\u5927\u4E8E\u6216\u7B49\u4E8E0"
        },
        "required_mark": "\u5FC5\u586B",
        "optional_mark": "\u9009\u586B",
        "required_fields_hint": "\u5E26 * \u7684\u4E3A\u5FC5\u586B\u9879\uFF0C\u63D0\u4EA4\u524D\u8BF7\u5B8C\u6574\u586B\u5199",
        "code": {
          "label": "\u81EA\u5B9A\u4E49\u8282\u70B9ID",
          "optional": "(\u9009\u586B)",
          "placeholder": "\u8BF7\u8F93\u5165\u81EA\u5B9A\u4E49\u8282\u70B9ID",
          "hint": "\u9009\u586B\uFF1B\u7528\u4E8E\u5916\u90E8\u7CFB\u7EDF\u5BF9\u7167\uFF0C\u4E0D\u5F71\u54CD\u534F\u8BAE\u8FDE\u63A5"
        },
        "tags": {
          "label": "\u8282\u70B9\u6807\u7B7E",
          "optional": "(\u9009\u586B)",
          "placeholder": "\u8F93\u5165\u540E\u6309\u9017\u53F7/\u56DE\u8F66\u6DFB\u52A0",
          "hint": "\u9009\u586B\uFF1B\u5C55\u793A\u5728\u7528\u6237\u7AEF\u8282\u70B9\u540D\u79F0\u65C1"
        },
        "groups": {
          "label": "\u6743\u9650\u7EC4",
          "add": "\u6DFB\u52A0\u6743\u9650\u7EC4",
          "placeholder": "\u8BF7\u9009\u62E9\u6743\u9650\u7EC4",
          "empty": "\u672A\u627E\u5230\u7ED3\u679C",
          "hint": "\u9009\u586B\uFF1B\u4E0D\u9009\u5219\u4EFB\u4F55\u6743\u9650\u7EC4\u7528\u6237\u90FD\u770B\u4E0D\u5230\u6B64\u8282\u70B9\uFF08\u53D6\u51B3\u4E8E\u4E1A\u52A1\u903B\u8F91\uFF0C\u5EFA\u8BAE\u81F3\u5C11\u9009\u4E00\u4E2A\uFF09"
        },
        "routes": {
          "label": "\u8DEF\u7531\u89C4\u5219",
          "empty": "\u672A\u627E\u5230\u8DEF\u7531\u89C4\u5219",
          "hint": "\u9009\u586B\uFF1B\u7ED1\u5B9A\u7684\u8DEF\u7531\u89C4\u5219\u4F1A\u4E0B\u53D1\u5230\u4F7F\u7528\u8BE5\u8282\u70B9\u7684\u5BA2\u6237\u7AEF\u8BA2\u9605\u914D\u7F6E\u4E2D"
        },
        "machine": {
          "label": "\u7ED1\u5B9A\u670D\u52A1\u5668",
          "placeholder": "\u9009\u62E9\u670D\u52A1\u5668\uFF08\u53EF\u9009\uFF09",
          "none": "\u72EC\u7ACB\u90E8\u7F72",
          "enabled_hint": "\u9009\u62E9\u662F\u5426\u7531\u6B64\u670D\u52A1\u5668\u7BA1\u7406\u8BE5\u8282\u70B9",
          "hint": "\u9009\u586B\uFF1B\u7ED1\u5B9A\u540E\u7531\u8BE5\u670D\u52A1\u5668\u4E0A\u7684 Fboard-Node \u6258\u7BA1\u6B64\u8282\u70B9"
        },
        "host": {
          "label": "\u8282\u70B9\u5730\u5740",
          "placeholder": "\u8BF7\u8F93\u5165\u8282\u70B9\u57DF\u540D\u6216\u8005IP",
          "error": "\u8282\u70B9\u5730\u5740\u4E0D\u80FD\u4E3A\u7A7A"
        },
        "port": {
          "label": "\u8FDE\u63A5\u7AEF\u53E3",
          "placeholder": "443 \u6216 10000-11000",
          "tooltip": "\u7528\u6237\u5B9E\u9645\u8FDE\u63A5\u4F7F\u7528\u7684\u7AEF\u53E3\u3002\u652F\u6301\u5355\u7AEF\u53E3\uFF08\u5982 443\uFF09\u6216\u7AEF\u53E3\u8303\u56F4\uFF08\u5982 10000-11000\uFF09\u3002Hysteria2 \u586B\u5199\u8303\u56F4\u65F6\u8282\u70B9\u4F1A\u5728\u8BE5\u8303\u56F4\u5185\u591A\u7AEF\u53E3\u76D1\u542C\u4EE5\u652F\u6301\u7AEF\u53E3\u8DF3\u8DC3\uFF08\u8DE8\u5EA6\u4E0A\u9650 1024\uFF09\uFF1B\u5176\u4ED6\u534F\u8BAE\u8303\u56F4\u4EC5\u7528\u4E8E\u8BA2\u9605\u968F\u673A\u9009\u7AEF\u53E3\u3002\u82E5\u4F7F\u7528\u4E2D\u8F6C/\u96A7\u9053\uFF0C\u53EF\u4E0E\u670D\u52A1\u7AEF\u53E3\u4E0D\u540C\u3002",
          "sync": "\u540C\u6B65\u5230\u670D\u52A1\u7AEF\u53E3",
          "error": "\u8FDE\u63A5\u7AEF\u53E3\u4E0D\u80FD\u4E3A\u7A7A"
        },
        "server_port": {
          "label": "\u670D\u52A1\u7AEF\u53E3",
          "placeholder": "\u8BF7\u8F93\u5165\u670D\u52A1\u7AEF\u53E3",
          "error": "\u670D\u52A1\u7AEF\u53E3\u4E0D\u80FD\u4E3A\u7A7A",
          "tooltip": "\u670D\u52A1\u5668\u4E0A\u7684\u5B9E\u9645\u76D1\u542C\u7AEF\u53E3\u3002Hysteria2 \u8FDE\u63A5\u7AEF\u53E3\u4E3A\u8303\u56F4\u65F6\uFF0C\u8282\u70B9\u6309\u8FDE\u63A5\u7AEF\u53E3\u8303\u56F4\u591A\u7AEF\u53E3\u76D1\u542C\uFF0C\u6B64\u5B57\u6BB5\u4F5C\u4E3A\u517C\u5BB9\u56DE\u9000\u7AEF\u53E3\u3002",
          "sync": "\u540C\u6B65\u5230\u670D\u52A1\u7AEF\u53E3"
        },
        "listen_address": {
          "label": "\u76D1\u542C\u5730\u5740",
          "placeholder": "\u7559\u7A7A\u4F7F\u7528\u9ED8\u8BA4 (0.0.0.0)\uFF0C\u6216\u8F93\u5165: 127.0.0.1, ::1 \u7B49",
          "description": "\u6307\u5B9A\u670D\u52A1\u5668\u76D1\u542C\u7684 IP \u5730\u5740\u3002\u7559\u7A7A\u5219\u4F7F\u7528\u9ED8\u8BA4\u5730\u5740 (0.0.0.0)\uFF0C\u8868\u793A\u76D1\u542C\u6240\u6709\u7F51\u7EDC\u63A5\u53E3\u3002\u53EF\u8BBE\u7F6E\u4E3A 127.0.0.1 (\u4EC5\u672C\u5730) \u6216\u7279\u5B9A IP \u5730\u5740",
          "show": "\u76D1\u542C\u5730\u5740",
          "hide": "\u9690\u85CF\u76D1\u542C\u5730\u5740",
          "optional": "\u53EF\u9009"
        },
        "parent": {
          "label": "\u7236\u7EA7\u8282\u70B9",
          "placeholder": "\u9009\u62E9\u7236\u8282\u70B9",
          "none": "\u65E0",
          "hint": "\u9009\u586B\uFF1B\u5B50\u8282\u70B9\u4F1A\u7EE7\u627F\u7236\u8282\u70B9\u534F\u8BAE\u914D\u7F6E\u4E0E\u5728\u7EBF\u72B6\u6001"
        },
        "virtualNode": {
          "label": "\u865A\u62DF\u8282\u70B9",
          "add": "\u6DFB\u52A0\u865A\u62DF\u8282\u70B9",
          "addDialogTitle": "\u6DFB\u52A0\u865A\u62DF\u8282\u70B9",
          "editDialogTitle": "\u7F16\u8F91\u865A\u62DF\u8282\u70B9",
          "edit": "\u7F16\u8F91",
          "delete": "\u5220\u9664",
          "save": "\u4FDD\u5B58\u865A\u62DF\u8282\u70B9",
          "saveSuccess": "\u865A\u62DF\u8282\u70B9\u5DF2\u4FDD\u5B58",
          "saveFailed": "\u4FDD\u5B58\u865A\u62DF\u8282\u70B9\u5931\u8D25",
          "description": "\u865A\u62DF\u8282\u70B9\u662F\u540C\u4E00\u670D\u52A1\u7684\u989D\u5916\u63A5\u5165\u70B9\uFF0C\u4E0E\u5B50\u7236\u8282\u70B9\u662F\u72EC\u7ACB\u529F\u80FD",
          "empty": "\u6682\u65E0\u865A\u62DF\u8282\u70B9",
          "tagsPlaceholder": "\u8F93\u5165\u540E\u6309\u56DE\u8F66\u6DFB\u52A0",
          "host": "\u4E3B\u673A\u5730\u5740",
          "hostPlaceholder": "\u4E3B\u673A\u540D / \u57DF\u540D",
          "port": "\u7AEF\u53E3",
          "groupIds": "\u6743\u9650\u7EC4",
          "groupIdsHint": "\u53EA\u80FD\u9009\u62E9\u7236\u8282\u70B9\u5DF2\u5305\u542B\u7684\u6743\u9650\u7EC4\uFF0C\u8D85\u51FA\u7236\u8282\u70B9\u5C06\u65E0\u6CD5\u8FDE\u63A5",
          "groupIdsParentEmpty": "\u8BF7\u5148\u4E3A\u7236\u8282\u70B9\u9009\u62E9\u6743\u9650\u7EC4",
          "tags": "\u6807\u7B7E",
          "show": "\u663E\u793A",
          "visible": "\u663E\u793A",
          "hidden": "\u9690\u85CF",
          "namePlaceholder": "\u865A\u62DF\u8282\u70B9\u540D\u79F0",
          "deleteConfirmTitle": "\u786E\u8BA4\u5220\u9664",
          "deleteConfirmDesc": "\u786E\u5B9A\u8981\u5220\u9664\u865A\u62DF\u8282\u70B9\u300C{{name}}\u300D\u5417\uFF1F\u6B64\u64CD\u4F5C\u4E0D\u53EF\u6062\u590D\u3002",
          "deleteSuccess": "\u865A\u62DF\u8282\u70B9\u5DF2\u5220\u9664",
          "deleteFailed": "\u5220\u9664\u5931\u8D25",
          "toggleFailed": "\u66F4\u65B0\u663E\u793A\u72B6\u6001\u5931\u8D25",
          "generateKeyPair": "\u751F\u6210\u5BC6\u94A5\u5BF9"
        },
        "route": {
          "label": "\u8DEF\u7531\u7EC4",
          "placeholder": "\u9009\u62E9\u8DEF\u7531\u7EC4",
          "empty": "\u672A\u627E\u5230\u7ED3\u679C"
        },
        "submit": "\u63D0\u4EA4",
        "cancel": "\u53D6\u6D88",
        "success": "\u63D0\u4EA4\u6210\u529F"
      },
      "networkTemplate": {
        "title": "\u7F51\u7EDC\u6A21\u677F",
        "empty": "\u5F53\u524D\u534F\u8BAE\u6682\u65E0\u53EF\u7528\u6A21\u677F",
        "description": "\u9009\u62E9\u4E00\u4E2A\u9884\u8BBE\u7F51\u7EDC\u6A21\u677F\uFF0C\u4E00\u952E\u586B\u5145\u534F\u8BAE\u914D\u7F6E",
        "use": "\u4F7F\u7528",
        "presets": {
          "vless-tcp-vision": {
            "label": "TCP + XTLS Vision",
            "description": "VLESS + TCP + XTLS Vision \u76F4\u8FDE\uFF0C\u901F\u5EA6\u6781\u5FEB\uFF0C\u63A8\u8350"
          },
          "vless-ws-tls": {
            "label": "WebSocket + TLS + CDN",
            "description": "VLESS + WebSocket + TLS\uFF0C\u53EF\u5957 CDN\uFF08Cloudflare \u7B49\uFF09"
          },
          "vless-grpc-tls": {
            "label": "gRPC + TLS",
            "description": "VLESS + gRPC + TLS\uFF0C\u9002\u5408\u5927\u89C4\u6A21\u8D1F\u8F7D"
          },
          "vless-tcp-reality": {
            "label": "TCP + REALITY",
            "description": "VLESS + REALITY \u76F4\u8FDE\uFF0C\u65E0\u9700\u8BC1\u4E66\uFF0C\u9632\u4E3B\u52A8\u63A2\u6D4B"
          },
          "trojan-tls": {
            "label": "Trojan + TLS",
            "description": "\u6807\u51C6 Trojan + TLS\uFF0C\u7AEF\u53E3 443"
          },
          "trojan-ws-tls": {
            "label": "Trojan + WebSocket + TLS",
            "description": "Trojan + WebSocket + TLS\uFF0C\u53EF\u5957 CDN"
          },
          "ss-simple": {
            "label": "\u6807\u51C6 AEAD",
            "description": "Shadowsocks \u52A0\u5BC6\u96A7\u9053\uFF0C\u7B80\u6D01\u9AD8\u6548"
          },
          "hy-standard": {
            "label": "Hysteria \u6807\u51C6",
            "description": "Hysteria \u57FA\u4E8E QUIC\uFF0C\u6297\u4E22\u5305\uFF0C\u9002\u5408\u5F31\u7F51"
          },
          "hy-brutal": {
            "label": "Hysteria2 Brute",
            "description": "Hysteria2 + Brute \u6A21\u5F0F\uFF0C\u6781\u9AD8\u5E26\u5BBD\u5229\u7528"
          },
          "tuic-v5": {
            "label": "TUIC v5 \u6807\u51C6",
            "description": "TUIC v5 \u57FA\u4E8E QUIC\uFF0C\u4F4E\u5EF6\u8FDF"
          },
          "anytls-default": {
            "label": "AnyTLS \u9ED8\u8BA4",
            "description": "AnyTLS \u81EA\u52A8\u5316 TLS \u4F2A\u88C5"
          },
          "sudoku-default": {
            "label": "Sudoku \u9ED8\u8BA4",
            "description": "\u4F4E\u71B5\u8868 + ChaCha20 + legacy HTTPMask"
          },
          "shadowquic-default": {
            "label": "ShadowQUIC \u9ED8\u8BA4",
            "description": "JLS \u4F2A\u88C5 + 0-RTT QUIC\uFF0C\u9ED8\u8BA4\u4E0A\u6E38 Cloudflare"
          }
        }
      },
      "dynamic_form": {
        "utls": {
          "fingerprint": {
            "random": "\u968F\u673A\uFF08\u8BA2\u9605\u65F6\u62BD\u53D6\uFF09",
            "randomized": "\u968F\u673A\u5316\uFF08\u5BA2\u6237\u7AEF\u5185\u968F\u673A\uFF09"
          }
        },
        "multiplex": {
          "enabled": {
            "label": "\u591A\u8DEF\u590D\u7528 (Multiplex)",
            "description": "\u901A\u8FC7\u5355\u6761 TCP \u8FDE\u63A5\u4F20\u8F93\u591A\u4E2A\u6D41\uFF0C\u964D\u4F4E\u63E1\u624B\u5EF6\u8FDF"
          },
          "protocol": {
            "label": "\u590D\u7528\u534F\u8BAE"
          },
          "max_connections": {
            "label": "\u6700\u5927\u8FDE\u63A5\u6570"
          },
          "min_streams": {
            "label": "\u6700\u5C0F\u6D41\u6570"
          },
          "padding": {
            "label": "\u542F\u7528\u586B\u5145"
          },
          "brutal": {
            "enabled": {
              "label": "TCP Brutal (\u6FC0\u8FDB\u62E5\u585E\u63A7\u5236)"
            },
            "up_mbps": {
              "label": "\u4E0A\u884C\u5E26\u5BBD"
            },
            "down_mbps": {
              "label": "\u4E0B\u884C\u5E26\u5BBD"
            },
            "description": "TCP Brutal \u662F\u53CC\u8FB9\u52A0\u901F\u7B97\u6CD5\uFF0C\u5EFA\u8BAE\u5E26\u5BBD\u8BBE\u4E3A\u673A\u5668\u5B9E\u9645\u5E26\u5BBD\u7684 80%-90%\uFF0C\u5F00\u542F\u540E BBR \u5C06\u5931\u6548\u3002"
          }
        },
        "ech": {
          "description": "\u4E3A\u652F\u6301\u7684 TLS \u5BA2\u6237\u7AEF\u542F\u7528 Encrypted Client Hello\u3002\u7559\u7A7A\u914D\u7F6E\u65F6\u4F1A\u5C1D\u8BD5\u901A\u8FC7 DNS \u67E5\u8BE2\u3002",
          "generate": "\u81EA\u52A8\u751F\u6210 ECH \u5BC6\u94A5\u5BF9",
          "generateSuccess": "ECH \u5BC6\u94A5\u5BF9\u5DF2\u751F\u6210",
          "generateFailed": "\u751F\u6210 ECH \u5BC6\u94A5\u5931\u8D25",
          "config": {
            "label": "ECH \u914D\u7F6E (PEM)",
            "placeholder": "\u7C98\u8D34 PEM \u683C\u5F0F\u7684 ECH \u914D\u7F6E\uFF0C\u6BCF\u884C\u4E00\u6BB5\u5185\u5BB9",
            "description": "\u7559\u7A7A\u65F6\uFF0Csing-box \u4F1A\u5C1D\u8BD5\u901A\u8FC7 DNS \u52A0\u8F7D ECH \u914D\u7F6E\u3002"
          },
          "config_path": {
            "label": "ECH \u914D\u7F6E\u6587\u4EF6\u8DEF\u5F84",
            "placeholder": "/etc/sing-box/ech.pem",
            "description": "\u6307\u5411 PEM \u683C\u5F0F ECH \u914D\u7F6E\u6587\u4EF6\u7684\u8DEF\u5F84\u3002"
          },
          "query_server_name": {
            "label": "ECH \u67E5\u8BE2\u57DF\u540D",
            "placeholder": "\u53EF\u9009\uFF0C\u7528\u4E8E\u8986\u76D6 HTTPS \u8BB0\u5F55\u67E5\u8BE2\u57DF\u540D",
            "description": "\u8986\u76D6\u7528\u4E8E ECH HTTPS \u8BB0\u5F55\u67E5\u8BE2\u7684\u57DF\u540D\uFF0C\u7559\u7A7A\u65F6\u9ED8\u8BA4\u4F7F\u7528 server_name\u3002"
          },
          "key": {
            "label": "ECH Key",
            "placeholder": "\u5F53\u540E\u7AEF\u9700\u8981\u65F6\u7C98\u8D34 ECH key \u5185\u5BB9",
            "description": "\u540E\u7AEF\u9700\u8981\u65F6\u53EF\u586B\u5199\u7684 ECH key \u5185\u5BB9\u3002"
          },
          "key_path": {
            "label": "ECH Key \u8DEF\u5F84",
            "placeholder": "/etc/sing-box/ech.key",
            "description": "\u540E\u7AEF\u9700\u8981\u65F6\u53EF\u586B\u5199\u7684 ECH key \u6587\u4EF6\u8DEF\u5F84\u3002"
          }
        },
        "anytls": {
          "tls": {
            "server_name": {
              "label": "\u670D\u52A1\u5668\u540D\u79F0\u6307\u793A(SNI)",
              "placeholder": "\u5F53\u8282\u70B9\u5730\u5740\u4E0E\u8BC1\u4E66\u4E0D\u4E00\u81F4\u65F6\u7528\u4E8E\u8BC1\u4E66\u9A8C\u8BC1"
            },
            "allow_insecure": "\u5141\u8BB8\u4E0D\u5B89\u5168\u8FDE\u63A5"
          },
          "padding_scheme": {
            "label": "\u586B\u5145\u65B9\u6848",
            "placeholder": "\u9009\u62E9\u586B\u5145\u65B9\u6848",
            "edit_btn": "\u7F16\u8F91\u586B\u5145\u65B9\u6848",
            "configured": "\u5DF2\u914D\u7F6E {{count}} \u6761\u89C4\u5219",
            "not_configured": "\u672A\u914D\u7F6E",
            "description": "\u7528\u4E8E\u6DF7\u6DC6\u6D41\u91CF\u7279\u5F81\u7684\u586B\u5145\u65B9\u6848\uFF0C\u6BCF\u884C\u4E00\u6761\u89C4\u5219\uFF0C\u652F\u6301\u901A\u914D\u7B26 *",
            "use_default": "\u4F7F\u7528\u9ED8\u8BA4\u65B9\u6848"
          }
        },
        "shadowsocks": {
          "cipher": {
            "label": "\u52A0\u5BC6\u7B97\u6CD5",
            "placeholder": "\u9009\u62E9\u52A0\u5BC6\u7B97\u6CD5",
            "search_placeholder": "\u641C\u7D22\u6216\u8F93\u5165\u81EA\u5B9A\u4E49\u52A0\u5BC6\u65B9\u5F0F...",
            "description": "\u9009\u62E9\u9884\u8BBE\u52A0\u5BC6\u65B9\u5F0F\u6216\u8F93\u5165\u81EA\u5B9A\u4E49\u52A0\u5BC6\u65B9\u5F0F",
            "preset_group": "\u9884\u8BBE\u52A0\u5BC6\u65B9\u5F0F",
            "custom_group": "\u81EA\u5B9A\u4E49\u52A0\u5BC6\u65B9\u5F0F",
            "current_value": "\u5F53\u524D\u503C",
            "use_custom": "\u4F7F\u7528",
            "no_results": "\u672A\u627E\u5230\u5339\u914D\u7684\u52A0\u5BC6\u65B9\u5F0F",
            "custom_hint": "\u4F60\u53EF\u4EE5\u76F4\u63A5\u8F93\u5165\u81EA\u5B9A\u4E49\u7684\u52A0\u5BC6\u65B9\u5F0F\uFF0C\u5982\uFF1Aaes-256-cfb",
            "custom_label": "\u81EA\u5B9A\u4E49"
          },
          "plugin": {
            "label": "\u63D2\u4EF6",
            "placeholder": "\u9009\u62E9\u63D2\u4EF6",
            "obfs_hint": "\u63D0\u793A\uFF1A\u914D\u7F6E\u683C\u5F0F\u5982 obfs=http;obfs-host=www.bing.com;path=/",
            "v2ray_hint": "\u63D0\u793A\uFF1AWebSocket\u6A21\u5F0F\u683C\u5F0F\u4E3A mode=websocket;host=mydomain.me;path=/;tls=true\uFF0CQUIC\u6A21\u5F0F\u683C\u5F0F\u4E3A mode=quic;host=mydomain.me",
            "gost_hint": "\u63D0\u793A\uFF1A\u914D\u7F6E\u683C\u5F0F\u5982 mode=websocket;host=mydomain.me;path=/;tls=true",
            "shadow_tls_hint": "\u63D0\u793A\uFF1A\u914D\u7F6E\u683C\u5F0F\u5982 host=cloud.tencent.com;password=auth_password;version=3",
            "restls_hint": "\u63D0\u793A\uFF1A\u914D\u7F6E\u683C\u5F0F\u5982 host=www.microsoft.com;password=auth_password;version-hint=tls13;restls-script=300?100<1,400~100",
            "kcptun_hint": "\u63D0\u793A\uFF1A\u914D\u7F6E\u683C\u5F0F\u5982 key=psk;crypt=aes-128-gcm;mode=fast;mtu=1350"
          },
          "plugin_opts": {
            "label": "\u63D2\u4EF6\u9009\u9879",
            "description": "\u6309\u7167 key=value;key2=value2 \u683C\u5F0F\u8F93\u5165\u63D2\u4EF6\u9009\u9879",
            "placeholder": "\u4F8B\u5982: mode=tls;host=bing.com"
          },
          "client_fingerprint": "\u5BA2\u6237\u7AEF\u6307\u7EB9",
          "client_fingerprint_placeholder": "\u9009\u62E9\u5BA2\u6237\u7AEF\u6307\u7EB9",
          "client_fingerprint_description": "\u5BA2\u6237\u7AEF\u4F2A\u88C5\u6307\u7EB9\uFF0C\u7528\u4E8E\u964D\u4F4E\u88AB\u8BC6\u522B\u98CE\u9669",
          "obfs": {
            "label": "\u6DF7\u6DC6",
            "placeholder": "\u9009\u62E9\u6DF7\u6DC6\u65B9\u5F0F",
            "none": "\u65E0",
            "http": "HTTP"
          },
          "obfs_settings": {
            "path": "\u8DEF\u5F84",
            "host": "Host"
          },
          "cert_config": {
            "tab": "TLS \u8BC1\u4E66",
            "cert_mode": {
              "label": "\u8BC1\u4E66\u6A21\u5F0F",
              "description": "\u9009\u62E9\u8BC1\u4E66\u7533\u8BF7\u65B9\u5F0F\uFF0C\u4EC5\u90E8\u5206\u540E\u7AEF\u8282\u70B9\u652F\u6301",
              "self_description": "\u81EA\u7B7E\u540D\u6A21\u5F0F\uFF1A\u4EC5\u9700\u586B\u5199\u57DF\u540D\uFF0C\u8BC1\u4E66\u7531\u8282\u70B9\u540E\u7AEF\u81EA\u52A8\u751F\u6210\uFF0810\u5E74\u6709\u6548\u671F\uFF09",
              "http_description": "HTTP-01 \u6A21\u5F0F\uFF1A\u9700\u8981 80 \u7AEF\u53E3\u53EF\u6B63\u5E38\u8BBF\u95EE\u4EE5\u5B8C\u6210\u8BA4\u8BC1",
              "dns_description": "DNS-01 \u6A21\u5F0F\uFF1A\u901A\u8FC7 DNS \u89E3\u6790\u8BB0\u5F55\u8BA4\u8BC1\uFF0C\u652F\u6301\u7533\u8BF7\u6CDB\u57DF\u540D\u8BC1\u4E66",
              "content_description": "\u5185\u5BB9\u63A8\u9001\u6A21\u5F0F\uFF1A\u76F4\u63A5\u5C06\u8BC1\u4E66\u5185\u5BB9\u4E0B\u53D1\u81F3\u8282\u70B9"
            },
            "domain": {
              "label": "\u8BC1\u4E66\u57DF\u540D",
              "placeholder": "example.com"
            },
            "email": {
              "label": "\u901A\u77E5\u90AE\u7BB1",
              "placeholder": "admin@example.com"
            },
            "http_port": {
              "label": "\u8BA4\u8BC1\u7AEF\u53E3",
              "description": "ACME \u8BA4\u8BC1\u7AEF\u53E3 (\u9ED8\u8BA4 80)"
            },
            "dns_provider": {
              "label": "DNS \u63D0\u4F9B\u5546",
              "doc_link": "\u67E5\u770B DNS \u63D0\u4F9B\u5546\u914D\u7F6E\u6307\u5357"
            },
            "dns_env": {
              "label": "\u73AF\u5883\u53D8\u91CF (API \u5BC6\u94A5)",
              "description_short": "\u6BCF\u884C\u4E00\u4E2A KEY=VALUE \u914D\u7F6E"
            },
            "cert_content": {
              "label": "\u8BC1\u4E66\u5185\u5BB9 (PEM)",
              "placeholder": "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----",
              "description": "\u7C98\u8D34\u5B8C\u6574\u8BC1\u4E66 PEM \u6B63\u6587\uFF0C\u4E0D\u662F\u6587\u4EF6\u8DEF\u5F84"
            },
            "key_content": {
              "label": "\u79C1\u94A5\u5185\u5BB9 (PEM)",
              "placeholder": "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----",
              "description": "\u7C98\u8D34\u5B8C\u6574\u79C1\u94A5 PEM \u6B63\u6587\uFF0C\u4E0D\u662F\u6587\u4EF6\u8DEF\u5F84"
            },
            "templates": {
              "title": "\u8BC1\u4E66\u6A21\u677F",
              "save_current": "\u4FDD\u5B58\u5F53\u524D\u4E3A\u6A21\u677F",
              "save": "\u4FDD\u5B58\u6A21\u677F",
              "saved": "\u8BC1\u4E66\u6A21\u677F\u5DF2\u4FDD\u5B58",
              "save_failed": "\u4FDD\u5B58\u8BC1\u4E66\u6A21\u677F\u5931\u8D25",
              "deleted": "\u8BC1\u4E66\u6A21\u677F\u5DF2\u5220\u9664",
              "delete_failed": "\u5220\u9664\u8BC1\u4E66\u6A21\u677F\u5931\u8D25",
              "delete": "\u5220\u9664\u6A21\u677F",
              "use": "\u4F7F\u7528",
              "applied": "\u5DF2\u5E94\u7528\u8BC1\u4E66\u6A21\u677F\uFF1A{{name}}",
              "name_required": "\u8BF7\u8F93\u5165\u6A21\u677F\u540D\u79F0",
              "name_placeholder": "\u6A21\u677F\u540D\u79F0\uFF0C\u4F8B\u5982\uFF1A\u751F\u4EA7\u8BC1\u4E66",
              "description_placeholder": "\u6A21\u677F\u63CF\u8FF0\uFF08\u53EF\u9009\uFF09",
              "search_placeholder": "\u641C\u7D22\u6A21\u677F\u540D\u79F0\u6216\u63CF\u8FF0\u2026",
              "empty_content": "\u8BF7\u5148\u586B\u5199\u8BC1\u4E66\u5185\u5BB9\u4E0E\u79C1\u94A5\u5185\u5BB9\uFF0C\u518D\u4FDD\u5B58\u4E3A\u6A21\u677F",
              "empty": "\u6682\u65E0\u8BC1\u4E66\u6A21\u677F"
            },
            "none_desc": "\u672A\u542F\u7528 TLS \u8BC1\u4E66\u914D\u7F6E"
          }
        },
        "vmess": {
          "tls": {
            "label": "TLS",
            "placeholder": "\u8BF7\u9009\u62E9\u5B89\u5168\u6027",
            "disabled": "\u4E0D\u652F\u6301",
            "enabled": "\u652F\u6301"
          },
          "tls_settings": {
            "server_name": {
              "label": "\u670D\u52A1\u5668\u540D\u79F0\u6307\u793A(SNI)",
              "placeholder": "\u4E0D\u4F7F\u7528\u8BF7\u7559\u7A7A"
            },
            "allow_insecure": "\u5141\u8BB8\u4E0D\u5B89\u5168?"
          },
          "network": {
            "label": "\u4F20\u8F93\u534F\u8BAE",
            "placeholder": "\u9009\u62E9\u4F20\u8F93\u534F\u8BAE"
          }
        },
        "trojan": {
          "server_name": {
            "label": "\u670D\u52A1\u5668\u540D\u79F0\u6307\u793A(SNI)",
            "placeholder": "\u5F53\u8282\u70B9\u5730\u5740\u4E8E\u8BC1\u4E66\u4E0D\u4E00\u81F4\u65F6\u7528\u4E8E\u8BC1\u4E66\u9A8C\u8BC1"
          },
          "allow_insecure": "\u5141\u8BB8\u4E0D\u5B89\u5168?",
          "reality_settings": {
            "server_name": {
              "label": "\u4F2A\u88C5\u7AD9\u70B9(dest)",
              "placeholder": "\u4F8B\u5982\uFF1Aexample.com"
            },
            "server_port": {
              "label": "\u7AEF\u53E3(port)",
              "placeholder": "\u4F8B\u5982\uFF1A443"
            },
            "allow_insecure": "\u5141\u8BB8\u4E0D\u5B89\u5168?",
            "private_key": {
              "label": "\u79C1\u94A5(Private key)"
            },
            "public_key": {
              "label": "\u516C\u94A5(Public key)"
            },
            "short_id": {
              "label": "Short ID",
              "placeholder": "\u53EF\u7559\u7A7A\uFF0C\u957F\u5EA6\u4E3A2\u7684\u500D\u6570\uFF0C\u6700\u957F16\u4F4D",
              "description": "\u5BA2\u6237\u7AEF\u53EF\u7528\u7684 shortId \u5217\u8868\uFF0C\u53EF\u7528\u4E8E\u533A\u5206\u4E0D\u540C\u7684\u5BA2\u6237\u7AEF\uFF0C\u4F7F\u75280-f\u7684\u5341\u516D\u8FDB\u5236\u5B57\u7B26",
              "generate": "\u751F\u6210 Short ID",
              "success": "Short ID \u751F\u6210\u6210\u529F"
            },
            "key_pair": {
              "generate": "\u751F\u6210\u5BC6\u94A5\u5BF9",
              "success": "\u5BC6\u94A5\u5BF9\u751F\u6210\u6210\u529F",
              "error": "\u751F\u6210\u5BC6\u94A5\u5BF9\u5931\u8D25"
            }
          },
          "network": {
            "label": "\u4F20\u8F93\u534F\u8BAE",
            "placeholder": "\u9009\u62E9\u4F20\u8F93\u534F\u8BAE"
          }
        },
        "hysteria": {
          "version": {
            "label": "\u534F\u8BAE\u7248\u672C",
            "placeholder": "\u534F\u8BAE\u7248\u672C"
          },
          "alpn": {
            "label": "ALPN",
            "placeholder": "ALPN"
          },
          "obfs": {
            "label": "\u6DF7\u6DC6",
            "type": {
              "label": "\u6DF7\u6DC6\u5B9E\u73B0",
              "placeholder": "\u9009\u62E9\u6DF7\u6DC6\u5B9E\u73B0",
              "salamander": "Salamander"
            },
            "password": {
              "label": "\u6DF7\u6DC6\u5BC6\u7801",
              "placeholder": "\u8BF7\u8F93\u5165\u6DF7\u6DC6\u5BC6\u7801",
              "generate_success": "\u6DF7\u6DC6\u5BC6\u7801\u751F\u6210\u6210\u529F"
            }
          },
          "tls": {
            "server_name": {
              "label": "\u670D\u52A1\u5668\u540D\u79F0\u6307\u793A(SNI)",
              "placeholder": "\u5F53\u8282\u70B9\u5730\u5740\u4E8E\u8BC1\u4E66\u4E0D\u4E00\u81F4\u65F6\u7528\u4E8E\u8BC1\u4E66\u9A8C\u8BC1"
            },
            "allow_insecure": "\u5141\u8BB8\u4E0D\u5B89\u5168?"
          },
          "bandwidth": {
            "up": {
              "label": "\u4E0A\u884C\u5BBD\u5E26",
              "placeholder": "\u8BF7\u8F93\u5165\u4E0A\u884C\u5BBD\u5E26",
              "suffix": "Mbps",
              "bbr_tip": "\uFF0C\u7559\u7A7A\u5219\u4F7F\u7528BBR"
            },
            "down": {
              "label": "\u4E0B\u884C\u5BBD\u5E26",
              "placeholder": "\u8BF7\u8F93\u5165\u4E0B\u884C\u5BBD\u5E26",
              "suffix": "Mbps",
              "bbr_tip": "\uFF0C\u7559\u7A7A\u5219\u4F7F\u7528BBR"
            }
          }
        },
        "vless": {
          "tls": {
            "label": "\u5B89\u5168\u6027",
            "placeholder": "\u8BF7\u9009\u62E9\u5B89\u5168\u6027",
            "none": "\u65E0",
            "tls": "TLS",
            "reality": "Reality"
          },
          "tls_settings": {
            "server_name": {
              "label": "\u670D\u52A1\u5668\u540D\u79F0\u6307\u793A(SNI)",
              "placeholder": "\u4E0D\u4F7F\u7528\u8BF7\u7559\u7A7A"
            },
            "allow_insecure": "\u5141\u8BB8\u4E0D\u5B89\u5168?"
          },
          "reality_settings": {
            "server_name": {
              "label": "\u4F2A\u88C5\u7AD9\u70B9(dest)",
              "placeholder": "\u4F8B\u5982\uFF1Aexample.com"
            },
            "server_port": {
              "label": "\u7AEF\u53E3(port)",
              "placeholder": "\u4F8B\u5982\uFF1A443"
            },
            "allow_insecure": "\u5141\u8BB8\u4E0D\u5B89\u5168?",
            "private_key": {
              "label": "\u79C1\u94A5(Private key)"
            },
            "public_key": {
              "label": "\u516C\u94A5(Public key)"
            },
            "short_id": {
              "label": "Short ID",
              "placeholder": "\u53EF\u7559\u7A7A\uFF0C\u957F\u5EA6\u4E3A2\u7684\u500D\u6570\uFF0C\u6700\u957F16\u4F4D",
              "description": "\u5BA2\u6237\u7AEF\u53EF\u7528\u7684 shortId \u5217\u8868\uFF0C\u53EF\u7528\u4E8E\u533A\u5206\u4E0D\u540C\u7684\u5BA2\u6237\u7AEF\uFF0C\u4F7F\u75280-f\u7684\u5341\u516D\u8FDB\u5236\u5B57\u7B26",
              "generate": "\u751F\u6210 Short ID",
              "success": "Short ID \u751F\u6210\u6210\u529F"
            },
            "key_pair": {
              "generate": "\u751F\u6210\u5BC6\u94A5\u5BF9",
              "success": "\u5BC6\u94A5\u5BF9\u751F\u6210\u6210\u529F",
              "error": "\u751F\u6210\u5BC6\u94A5\u5BF9\u5931\u8D25"
            }
          },
          "network": {
            "label": "\u4F20\u8F93\u534F\u8BAE",
            "placeholder": "\u9009\u62E9\u4F20\u8F93\u534F\u8BAE"
          },
          "flow": {
            "label": "\u6D41\u63A7",
            "placeholder": "\u9009\u62E9\u6D41\u63A7"
          },
          "encryption": {
            "label": "VLESS Encryption",
            "description": "\u542F\u7528 VLESS \u52A0\u5BC6",
            "server_label": "decryption",
            "server_placeholder": "./xray vlessenc \u751F\u6210",
            "server_description": "\u670D\u52A1\u7AEF decryption \u53C2\u6570\uFF0C\u53EF\u7531 ./xray vlessenc \u751F\u6210",
            "client_label": "encryption",
            "client_placeholder": "./xray vlessenc \u751F\u6210",
            "client_description": "\u5BA2\u6237\u7AEF encryption \u53C2\u6570\uFF0C\u9700\u4E0E\u670D\u52A1\u7AEF\u914D\u5BF9",
            "generate_hint": "./xray vlessenc \u751F\u6210"
          }
        },
        "tuic": {
          "version": {
            "label": "\u534F\u8BAE\u7248\u672C",
            "placeholder": "\u9009\u62E9TUIC\u7248\u672C"
          },
          "password": {
            "label": "\u5BC6\u7801",
            "placeholder": "\u8BF7\u8F93\u5165\u5BC6\u7801",
            "generate_success": "\u5BC6\u7801\u751F\u6210\u6210\u529F"
          },
          "congestion_control": {
            "label": "\u62E5\u585E\u63A7\u5236",
            "placeholder": "\u9009\u62E9\u62E5\u585E\u63A7\u5236\u7B97\u6CD5"
          },
          "udp_relay_mode": {
            "label": "UDP\u4E2D\u7EE7\u6A21\u5F0F",
            "placeholder": "\u9009\u62E9UDP\u4E2D\u7EE7\u6A21\u5F0F"
          },
          "tls": {
            "server_name": {
              "label": "\u670D\u52A1\u5668\u540D\u79F0\u6307\u793A(SNI)",
              "placeholder": "\u5F53\u8282\u70B9\u5730\u5740\u4E0E\u8BC1\u4E66\u4E0D\u4E00\u81F4\u65F6\u7528\u4E8E\u8BC1\u4E66\u9A8C\u8BC1"
            },
            "allow_insecure": "\u5141\u8BB8\u4E0D\u5B89\u5168?",
            "alpn": {
              "label": "ALPN",
              "placeholder": "\u9009\u62E9ALPN\u534F\u8BAE",
              "empty": "\u672A\u627E\u5230\u53EF\u7528\u7684ALPN\u534F\u8BAE"
            }
          }
        },
        "socks": {
          "version": {
            "label": "\u534F\u8BAE\u7248\u672C",
            "placeholder": "\u9009\u62E9SOCKS\u7248\u672C"
          },
          "tls": {
            "label": "TLS",
            "placeholder": "\u8BF7\u9009\u62E9\u5B89\u5168\u6027",
            "disabled": "\u4E0D\u652F\u6301",
            "enabled": "\u652F\u6301"
          },
          "tls_settings": {
            "server_name": {
              "label": "\u670D\u52A1\u5668\u540D\u79F0\u6307\u793A(SNI)",
              "placeholder": "\u4E0D\u4F7F\u7528\u8BF7\u7559\u7A7A"
            },
            "allow_insecure": "\u5141\u8BB8\u4E0D\u5B89\u5168?"
          },
          "network": {
            "label": "\u4F20\u8F93\u534F\u8BAE",
            "placeholder": "\u9009\u62E9\u4F20\u8F93\u534F\u8BAE"
          }
        },
        "sudoku": {
          "name": "Sudoku",
          "keyPairTitle": "Sudoku \u5BC6\u94A5\u5BF9",
          "keyPairDescription": "\u670D\u52A1\u7AEF\u4F7F\u7528 Master Public Key\uFF1BMaster Private Key \u4EC5\u4FDD\u5B58\u5728\u9762\u677F\u7528\u4E8E\u6D3E\u751F\u7528\u6237\u5BC6\u94A5\uFF0C\u4E0D\u4F1A\u4E0B\u53D1\u5230\u8282\u70B9",
          "generate": "\u4E00\u952E\u751F\u6210",
          "generateSuccess": "Sudoku \u5BC6\u94A5\u5BF9\u5DF2\u751F\u6210",
          "generateFailed": "\u751F\u6210 Sudoku \u5BC6\u94A5\u5931\u8D25",
          "publicPlaceholder": "\u70B9\u51FB\u53F3\u4E0A\u89D2\u300C\u4E00\u952E\u751F\u6210\u300D\u81EA\u52A8\u586B\u5199",
          "privatePlaceholder": "\u4EC5\u9762\u677F\u4FDD\u5B58\uFF0C\u52FF\u6CC4\u9732",
          "master_public_key": "Master \u516C\u94A5",
          "master_private_key": "Master \u79C1\u94A5",
          "aead_method": "AEAD \u7B97\u6CD5",
          "padding_min": "\u6700\u5C0F\u586B\u5145\u7387",
          "padding_max": "\u6700\u5927\u586B\u5145\u7387",
          "table_type": "\u8868\u7C7B\u578B",
          "enable_pure_downlink": "\u7EAF Sudoku \u4E0B\u884C",
          "custom_table": "\u81EA\u5B9A\u4E49\u8868",
          "custom_tables": "\u81EA\u5B9A\u4E49\u8868\u5217\u8868",
          "handshake_timeout": "\u63E1\u624B\u8D85\u65F6",
          "fallback": "\u56DE\u843D\u5730\u5740",
          "multiplex": "\u591A\u8DEF\u590D\u7528",
          "httpmask": "HTTPMask"
        },
        "naive": {
          "tls_settings": {
            "server_name": {
              "label": "\u670D\u52A1\u5668\u540D\u79F0\u6307\u793A(SNI)",
              "placeholder": "\u4E0D\u4F7F\u7528\u8BF7\u7559\u7A7A"
            },
            "allow_insecure": "\u5141\u8BB8\u4E0D\u5B89\u5168?"
          },
          "tls": {
            "label": "TLS",
            "placeholder": "\u8BF7\u9009\u62E9\u5B89\u5168\u6027",
            "disabled": "\u4E0D\u652F\u6301",
            "enabled": "\u652F\u6301",
            "server_name": {
              "label": "\u670D\u52A1\u5668\u540D\u79F0\u6307\u793A(SNI)",
              "placeholder": "\u5F53\u8282\u70B9\u5730\u5740\u4E0E\u8BC1\u4E66\u4E0D\u4E00\u81F4\u65F6\u7528\u4E8E\u8BC1\u4E66\u9A8C\u8BC1"
            },
            "allow_insecure": "\u5141\u8BB8\u4E0D\u5B89\u5168\u8FDE\u63A5"
          }
        },
        "http": {
          "tls": {
            "label": "TLS",
            "placeholder": "\u8BF7\u9009\u62E9\u5B89\u5168\u6027",
            "disabled": "\u4E0D\u652F\u6301",
            "enabled": "\u652F\u6301",
            "server_name": {
              "label": "\u670D\u52A1\u5668\u540D\u79F0\u6307\u793A(SNI)",
              "placeholder": "\u5F53\u8282\u70B9\u5730\u5740\u4E0E\u8BC1\u4E66\u4E0D\u4E00\u81F4\u65F6\u7528\u4E8E\u8BC1\u4E66\u9A8C\u8BC1"
            },
            "allow_insecure": "\u5141\u8BB8\u4E0D\u5B89\u5168\u8FDE\u63A5"
          },
          "tls_settings": {
            "server_name": {
              "label": "\u670D\u52A1\u5668\u540D\u79F0\u6307\u793A(SNI)",
              "placeholder": "\u5F53\u8282\u70B9\u5730\u5740\u4E0E\u8BC1\u4E66\u4E0D\u4E00\u81F4\u65F6\u7528\u4E8E\u8BC1\u4E66\u9A8C\u8BC1"
            },
            "allow_insecure": "\u5141\u8BB8\u4E0D\u5B89\u5168\u8FDE\u63A5"
          }
        },
        "mieru": {
          "transport": {
            "label": "\u4F20\u8F93\u534F\u8BAE",
            "placeholder": "\u9009\u62E9\u4F20\u8F93\u534F\u8BAE"
          },
          "traffic_pattern": {
            "label": "\u6D41\u91CF\u7279\u5F81\u4F2A\u88C5 (Base64)",
            "placeholder": "\u7559\u7A7A\u4F7F\u7528\u9ED8\u8BA4\uFF1B\u6216\u70B9\u51FB\u53F3\u4FA7\u6309\u94AE\u751F\u6210",
            "description": "\u53EF\u9009\u3002\u5B98\u65B9 mieru Traffic Pattern \u7684 Base64 \u4E32\uFF0C\u7528\u4E8E TCP \u5206\u7247 / Nonce \u524D\u7F00\u7B49\u6297 DPI \u6D41\u91CF\u6574\u5F62\uFF1B\u53EF\u7528 mita/mieru export traffic-pattern \u5BFC\u51FA\u3002",
            "generate": "\u751F\u6210\u6D41\u91CF\u7279\u5F81\u4F2A\u88C5",
            "success": "\u6D41\u91CF\u7279\u5F81\u4F2A\u88C5\u5DF2\u751F\u6210"
          }
        },
        "trusttunnel": {
          "client_random_auth_key": {
            "generate": "\u751F\u6210\u968F\u673A Auth Key\uFF0816 \u5B57\u8282 hex\uFF09"
          }
        },
        "cert_config": {
          "tab": "TLS \u8BC1\u4E66",
          "none_desc": "\u672A\u542F\u7528 TLS \u8BC1\u4E66\u914D\u7F6E",
          "cert_mode": {
            "label": "\u8BC1\u4E66\u6A21\u5F0F",
            "description": "\u9009\u62E9\u8BC1\u4E66\u7533\u8BF7\u65B9\u5F0F\uFF0C\u4EC5\u90E8\u5206\u540E\u7AEF\u8282\u70B9\u652F\u6301",
            "none_desc": "\u672A\u542F\u7528 TLS \u8BC1\u4E66\u914D\u7F6E"
          },
          "domain": {
            "label": "\u8BC1\u4E66\u57DF\u540D"
          },
          "email": {
            "label": "\u901A\u77E5\u90AE\u7BB1"
          },
          "http_port": {
            "label": "\u6311\u6218\u7AEF\u53E3",
            "description": "ACME \u6311\u6218\u7AEF\u53E3 (\u9ED8\u8BA4 80)"
          },
          "dns_provider": {
            "label": "DNS \u63D0\u4F9B\u5546",
            "doc_link": "\u67E5\u770B DNS \u63D0\u4F9B\u5546\u914D\u7F6E\u6307\u5357"
          },
          "dns_env": {
            "label": "\u73AF\u5883\u53D8\u91CF (API \u5BC6\u94A5)",
            "description_short": "\u6BCF\u884C\u4E00\u4E2A KEY=VALUE \u914D\u7F6E"
          },
          "cert_content": {
            "label": "\u8BC1\u4E66\u5185\u5BB9 (PEM)",
            "placeholder": "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----",
            "description": "\u7C98\u8D34\u5B8C\u6574\u8BC1\u4E66 PEM \u6B63\u6587\uFF0C\u4E0D\u662F\u6587\u4EF6\u8DEF\u5F84"
          },
          "key_content": {
            "label": "\u79C1\u94A5\u5185\u5BB9 (PEM)",
            "placeholder": "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----",
            "description": "\u7C98\u8D34\u5B8C\u6574\u79C1\u94A5 PEM \u6B63\u6587\uFF0C\u4E0D\u662F\u6587\u4EF6\u8DEF\u5F84"
          },
          "templates": {
            "title": "\u8BC1\u4E66\u6A21\u677F",
            "save_current": "\u4FDD\u5B58\u5F53\u524D\u4E3A\u6A21\u677F",
            "save": "\u4FDD\u5B58\u6A21\u677F",
            "saved": "\u8BC1\u4E66\u6A21\u677F\u5DF2\u4FDD\u5B58",
            "save_failed": "\u4FDD\u5B58\u8BC1\u4E66\u6A21\u677F\u5931\u8D25",
            "deleted": "\u8BC1\u4E66\u6A21\u677F\u5DF2\u5220\u9664",
            "delete_failed": "\u5220\u9664\u8BC1\u4E66\u6A21\u677F\u5931\u8D25",
            "delete": "\u5220\u9664\u6A21\u677F",
            "use": "\u4F7F\u7528",
            "applied": "\u5DF2\u5E94\u7528\u8BC1\u4E66\u6A21\u677F\uFF1A{{name}}",
            "name_required": "\u8BF7\u8F93\u5165\u6A21\u677F\u540D\u79F0",
            "name_placeholder": "\u6A21\u677F\u540D\u79F0\uFF0C\u4F8B\u5982\uFF1A\u751F\u4EA7\u8BC1\u4E66",
            "description_placeholder": "\u6A21\u677F\u63CF\u8FF0\uFF08\u53EF\u9009\uFF09",
            "search_placeholder": "\u641C\u7D22\u6A21\u677F\u540D\u79F0\u6216\u63CF\u8FF0\u2026",
            "empty_content": "\u8BF7\u5148\u586B\u5199\u8BC1\u4E66\u5185\u5BB9\u4E0E\u79C1\u94A5\u5185\u5BB9\uFF0C\u518D\u4FDD\u5B58\u4E3A\u6A21\u677F",
            "empty": "\u6682\u65E0\u8BC1\u4E66\u6A21\u677F"
          }
        },
        "routing": {
          "outbounds_tab": "\u81EA\u5B9A\u4E49 Outbounds",
          "routes_tab": "\u81EA\u5B9A\u4E49 Routes",
          "outbounds": "\u81EA\u5B9A\u4E49 Outbounds (JSON)",
          "routes": "\u81EA\u5B9A\u4E49 Routes (JSON)",
          "error": {
            "must_be_array": "\u5FC5\u987B\u662F\u4E00\u4E2A JSON \u6570\u7EC4 []",
            "invalid_json": "\u65E0\u6548\u7684 JSON \u683C\u5F0F"
          }
        },
        "advanced": {
          "trigger_label": "\u9AD8\u7EA7\u8BBE\u7F6E",
          "dialog_title": "\u9AD8\u7EA7\u534F\u8BAE\u914D\u7F6E",
          "tls_tab": "TLS",
          "route_tab": "\u8DEF\u7531",
          "multiplex_tab": "\u591A\u8DEF\u590D\u7528"
        }
      },
      "network_settings": {
        "edit_protocol": "\u7F16\u8F91\u534F\u8BAE",
        "edit_protocol_config": "\u7F16\u8F91\u7F51\u7EDC\u8BBE\u7F6E",
        "edit_padding_scheme": "\u7F16\u8F91\u586B\u5145\u65B9\u6848",
        "use_template": "\u4F7F\u7528{{template}}\u6A21\u677F",
        "json_label": "\u7F51\u7EDC\u8BBE\u7F6E (JSON)",
        "json_config_placeholder": "\u8BF7\u8F93\u5165 JSON \u683C\u5F0F\u7684\u7F51\u7EDC\u8BBE\u7F6E",
        "json_config_placeholder_with_template": "\u8BF7\u8F93\u5165 JSON \u914D\u7F6E\uFF0C\u4F8B\u5982 path / host / serviceName \u7B49",
        "templates": "\u6A21\u677F\u5217\u8868",
        "templates_empty": "\u6682\u65E0\u53EF\u7528\u6A21\u677F\uFF0C\u53EF\u5C06\u5F53\u524D\u914D\u7F6E\u4FDD\u5B58\u4E3A\u6A21\u677F",
        "templates_custom": "\u6211\u7684\u6A21\u677F",
        "templates_builtin": "\u5185\u7F6E\u6A21\u677F",
        "builtin": "\u5185\u7F6E",
        "save_as_template": "\u4FDD\u5B58\u5F53\u524D\u4E3A\u6A21\u677F",
        "save_template": "\u4FDD\u5B58\u6A21\u677F",
        "delete_template": "\u5220\u9664\u6A21\u677F",
        "use_template_btn": "\u4F7F\u7528",
        "template_name_placeholder": "\u6A21\u677F\u540D\u79F0\uFF0C\u4F8B\u5982\uFF1AWS \u81EA\u5B9A\u4E49\u8DEF\u5F84",
        "template_name_required": "\u8BF7\u8F93\u5165\u6A21\u677F\u540D\u79F0",
        "template_empty": "\u5F53\u524D\u914D\u7F6E\u4E3A\u7A7A\uFF0C\u65E0\u6CD5\u4FDD\u5B58\u4E3A\u6A21\u677F",
        "template_from_current": "\u6765\u81EA\u5F53\u524D {{network}} \u914D\u7F6E",
        "template_custom_desc": "\u81EA\u5B9A\u4E49\u7F51\u7EDC\u8BBE\u7F6E\u6A21\u677F",
        "template_saved": "\u6A21\u677F\u5DF2\u4FDD\u5B58",
        "template_save_failed": "\u4FDD\u5B58\u6A21\u677F\u5931\u8D25",
        "template_deleted": "\u6A21\u677F\u5DF2\u5220\u9664",
        "template_delete_failed": "\u5220\u9664\u6A21\u677F\u5931\u8D25",
        "template_applied": "\u5DF2\u5E94\u7528\u6A21\u677F\uFF1A{{name}}",
        "validation": {
          "must_be_array": "\u914D\u7F6E\u5FC5\u987B\u662F\u4E00\u4E2AJSON\u6570\u7EC4",
          "must_be_object": "\u914D\u7F6E\u5FC5\u987B\u662F\u4E00\u4E2AJSON\u5BF9\u8C61",
          "invalid_json": "\u65E0\u6548\u7684JSON\u683C\u5F0F"
        },
        "errors": {
          "save_failed": "\u4FDD\u5B58\u65F6\u53D1\u751F\u9519\u8BEF"
        },
        "builtin_templates": {
          "ws-basic": {
            "name": "WebSocket \u57FA\u7840",
            "description": "path + Host"
          },
          "ws-cdn": {
            "name": "WebSocket + CDN",
            "description": "\u9002\u5408 Cloudflare \u7B49 CDN"
          },
          "grpc-basic": {
            "name": "gRPC \u57FA\u7840",
            "description": "serviceName"
          },
          "grpc-multi": {
            "name": "gRPC multiMode",
            "description": "\u5F00\u542F multiMode"
          },
          "http-basic": {
            "name": "HTTP/2 \u57FA\u7840",
            "description": "path + host"
          },
          "tcp-none": {
            "name": "TCP \u65E0\u4F2A\u88C5",
            "description": "\u7EAF TCP"
          },
          "tcp-http": {
            "name": "TCP HTTP \u4F2A\u88C5",
            "description": "header type = http"
          },
          "xhttp-basic": {
            "name": "XHTTP \u57FA\u7840",
            "description": "path \u6A21\u5F0F"
          },
          "quic-basic": {
            "name": "QUIC \u57FA\u7840",
            "description": "security + key"
          },
          "kcp-basic": {
            "name": "KCP \u57FA\u7840",
            "description": "mtu / tti \u9ED8\u8BA4"
          }
        }
      },
      "common": {
        "cancel": "\u53D6\u6D88",
        "confirm": "\u786E\u5B9A"
      },
      "description": "\u914D\u7F6E\u8282\u70B9\u901A\u4FE1\u4E0E\u540C\u6B65\u8BBE\u7F6E\uFF0C\u5305\u62EC\u901A\u4FE1\u5BC6\u94A5\u3001\u8F6E\u8BE2\u95F4\u9694\u3001\u8D1F\u8F7D\u5747\u8861\u7B49\u9AD8\u7EA7\u9009\u9879\u3002",
      "device_limit_mode": {
        "description": "\u5BBD\u677E\u6A21\u5F0F\u4E0B\uFF0C\u540C\u4E00 IP \u5730\u5740\u7684\u591A\u4E2A\u8282\u70B9\u8BA1\u4E3A 1 \u53F0\u8BBE\u5907\u3002",
        "placeholder": "\u8BF7\u9009\u62E9\u8BBE\u5907\u9650\u5236\u6A21\u5F0F",
        "relaxed": "\u5BBD\u677E\u6A21\u5F0F",
        "strict": "\u4E25\u683C\u6A21\u5F0F",
        "title": "\u8BBE\u5907\u9650\u5236\u6A21\u5F0F"
      },
      "saving": "\u4FDD\u5B58\u4E2D...",
      "server_pull_interval": {
        "description": "\u8282\u70B9\u4ECE\u9762\u677F\u62C9\u53D6\u6570\u636E\u7684\u9891\u7387\u3002",
        "placeholder": "\u8BF7\u8F93\u5165\u62C9\u53D6\u95F4\u9694",
        "title": "\u8282\u70B9\u62C9\u53D6\u8F6E\u8BE2\u95F4\u9694"
      },
      "server_push_interval": {
        "description": "\u8282\u70B9\u5411\u9762\u677F\u63A8\u9001\u6570\u636E\u7684\u9891\u7387\u3002",
        "placeholder": "\u8BF7\u8F93\u5165\u63A8\u9001\u95F4\u9694",
        "title": "\u8282\u70B9\u63A8\u9001\u8F6E\u8BE2\u95F4\u9694"
      },
      "server_token": {
        "description": "\u9762\u677F\u4E0E\u8282\u70B9\u901A\u4FE1\u7684\u5BC6\u94A5\uFF0C\u7528\u4E8E\u9632\u6B62\u672A\u6388\u6743\u8BBF\u95EE\u3002",
        "placeholder": "\u8BF7\u8F93\u5165\u901A\u4FE1\u5BC6\u94A5",
        "title": "\u901A\u4FE1\u5BC6\u94A5"
      },
      "title": "\u8282\u70B9\u914D\u7F6E",
      "messages": {
        "saveFailed": "\u4FDD\u5B58\u5931\u8D25"
      }
    },
    "coupon": {
      "title": "\u4F18\u60E0\u5238\u7BA1\u7406",
      "description": "\u5728\u8FD9\u91CC\u53EF\u4EE5\u67E5\u770B\u4F18\u60E0\u5238\uFF0C\u5305\u62EC\u589E\u52A0\u3001\u67E5\u770B\u3001\u5220\u9664\u7B49\u64CD\u4F5C\u3002",
      "table": {
        "columns": {
          "id": "ID",
          "select": "\u9009\u62E9",
          "show": "\u542F\u7528",
          "name": "\u5377\u540D\u79F0",
          "type": "\u7C7B\u578B",
          "code": "\u5377\u7801",
          "limitUse": "\u5269\u4F59\u6B21\u6570",
          "limitUseWithUser": "\u53EF\u7528\u6B21\u6570/\u7528\u6237",
          "validity": "\u6709\u6548\u671F",
          "actions": "\u64CD\u4F5C"
        },
        "validity": {
          "expired": "\u5DF2\u8FC7\u671F{{days}}\u5929",
          "notStarted": "{{days}}\u5929\u540E\u5F00\u59CB",
          "remaining": "\u5269\u4F59{{days}}\u5929",
          "startTime": "\u5F00\u59CB\u65F6\u95F4",
          "endTime": "\u7ED3\u675F\u65F6\u95F4",
          "unlimited": "\u65E0\u9650\u6B21",
          "noLimit": "\u65E0\u9650\u5236"
        },
        "actions": {
          "edit": "\u7F16\u8F91",
          "delete": "\u5220\u9664",
          "deleteConfirm": {
            "title": "\u786E\u8BA4\u5220\u9664",
            "description": "\u6B64\u64CD\u4F5C\u5C06\u6C38\u4E45\u5220\u9664\u8BE5\u4F18\u60E0\u5238\uFF0C\u5220\u9664\u540E\u65E0\u6CD5\u6062\u590D\u3002\u786E\u5B9A\u8981\u7EE7\u7EED\u5417\uFF1F",
            "confirmText": "\u5220\u9664"
          },
          "batchDelete": "\u6279\u91CF\u5220\u9664 ({{count}})",
          "batchDeleteConfirm": {
            "title": "\u6279\u91CF\u5220\u9664",
            "description": "\u786E\u5B9A\u8981\u5220\u9664\u9009\u4E2D\u7684 {{count}} \u5F20\u4F18\u60E0\u5238\u5417\uFF1F\u5220\u9664\u540E\u65E0\u6CD5\u6062\u590D\u3002"
          },
          "dropExpired": "\u5220\u9664\u5DF2\u8FC7\u671F",
          "dropExpiredConfirm": {
            "title": "\u5220\u9664\u5DF2\u8FC7\u671F\u4F18\u60E0\u5238",
            "description": "\u786E\u5B9A\u8981\u5220\u9664\u6240\u6709\u5DF2\u8FC7\u671F\u7684\u4F18\u60E0\u5238\u5417\uFF1F\u5220\u9664\u540E\u65E0\u6CD5\u6062\u590D\u3002"
          },
          "dropExpiredResult": {
            "success": "\u5DF2\u5220\u9664 {{count}} \u5F20\u8FC7\u671F\u4F18\u60E0\u5238",
            "empty": "\u6CA1\u6709\u627E\u5230\u5DF2\u8FC7\u671F\u7684\u4F18\u60E0\u5238"
          },
          "dropDepleted": "\u5220\u9664\u5DF2\u7528\u5C3D",
          "dropDepletedConfirm": {
            "title": "\u5220\u9664\u5DF2\u7528\u5C3D\u4F18\u60E0\u5238",
            "description": "\u786E\u5B9A\u8981\u5220\u9664\u6240\u6709\u5269\u4F59\u6B21\u6570\u4E3A 0 \u7684\u4F18\u60E0\u5238\u5417\uFF1F\u5220\u9664\u540E\u65E0\u6CD5\u6062\u590D\u3002"
          },
          "dropDepletedResult": {
            "success": "\u5DF2\u5220\u9664 {{count}} \u5F20\u5DF2\u7528\u5C3D\u4F18\u60E0\u5238",
            "empty": "\u6CA1\u6709\u627E\u5230\u5269\u4F59\u6B21\u6570\u4E3A 0 \u7684\u4F18\u60E0\u5238"
          }
        },
        "toolbar": {
          "search": "\u641C\u7D22\u4F18\u60E0\u5238...",
          "type": "\u7C7B\u578B",
          "reset": "\u91CD\u7F6E",
          "types": {
            "1": "\u6309\u91D1\u989D\u4F18\u60E0",
            "2": "\u6309\u6BD4\u4F8B\u4F18\u60E0"
          }
        }
      },
      "form": {
        "add": "\u6DFB\u52A0\u4F18\u60E0\u5238",
        "edit": "\u7F16\u8F91\u4F18\u60E0\u5238",
        "name": {
          "label": "\u4F18\u60E0\u5238\u540D\u79F0",
          "placeholder": "\u8BF7\u8F93\u5165\u4F18\u60E0\u5238\u540D\u79F0",
          "required": "\u8BF7\u8F93\u5165\u4F18\u60E0\u5238\u540D\u79F0"
        },
        "type": {
          "label": "\u4F18\u60E0\u5238\u7C7B\u578B\u548C\u503C",
          "placeholder": "\u4F18\u60E0\u5238\u7C7B\u578B"
        },
        "value": {
          "placeholder": "\u8BF7\u8F93\u5165\u91D1\u989D\uFF08\u5143\uFF09"
        },
        "validity": {
          "label": "\u4F18\u60E0\u5238\u6709\u6548\u671F",
          "to": "\u81F3",
          "endTimeError": "\u7ED3\u675F\u65F6\u95F4\u5FC5\u987B\u665A\u4E8E\u5F00\u59CB\u65F6\u95F4"
        },
        "limitUse": {
          "label": "\u6700\u5927\u4F7F\u7528\u6B21\u6570",
          "placeholder": "\u9650\u5236\u6700\u5927\u4F7F\u7528\u6B21\u6570\uFF0C\u7559\u7A7A\u5219\u4E0D\u9650\u5236",
          "description": "\u8BBE\u7F6E\u4F18\u60E0\u5238\u7684\u603B\u4F7F\u7528\u6B21\u6570\u9650\u5236\uFF0C\u7559\u7A7A\u8868\u793A\u4E0D\u9650\u5236\u4F7F\u7528\u6B21\u6570"
        },
        "limitUseWithUser": {
          "label": "\u6BCF\u4E2A\u7528\u6237\u53EF\u4F7F\u7528\u6B21\u6570",
          "placeholder": "\u9650\u5236\u6BCF\u4E2A\u7528\u6237\u53EF\u4F7F\u7528\u6B21\u6570\uFF0C\u7559\u7A7A\u5219\u4E0D\u9650\u5236",
          "description": "\u9650\u5236\u6BCF\u4E2A\u7528\u6237\u53EF\u4F7F\u7528\u8BE5\u4F18\u60E0\u5238\u7684\u6B21\u6570\uFF0C\u7559\u7A7A\u8868\u793A\u4E0D\u9650\u5236\u5355\u7528\u6237\u4F7F\u7528\u6B21\u6570"
        },
        "limitPeriod": {
          "label": "\u6307\u5B9A\u5468\u671F",
          "placeholder": "\u9650\u5236\u6307\u5B9A\u5468\u671F\u53EF\u4EE5\u4F7F\u7528\u4F18\u60E0\uFF0C\u7559\u7A7A\u5219\u4E0D\u9650\u5236",
          "description": "\u9009\u62E9\u53EF\u4EE5\u4F7F\u7528\u4F18\u60E0\u5238\u7684\u8BA2\u9605\u5468\u671F\uFF0C\u7559\u7A7A\u8868\u793A\u4E0D\u9650\u5236\u4F7F\u7528\u5468\u671F",
          "empty": "\u6CA1\u6709\u627E\u5230\u5339\u914D\u7684\u5468\u671F"
        },
        "limitPlan": {
          "label": "\u6307\u5B9A\u8BA2\u9605",
          "placeholder": "\u9650\u5236\u6307\u5B9A\u8BA2\u9605\u53EF\u4EE5\u4F7F\u7528\u4F18\u60E0\uFF0C\u7559\u7A7A\u5219\u4E0D\u9650\u5236",
          "description": "\u9009\u62E9\u53EF\u4EE5\u4F7F\u7528\u4F18\u60E0\u5238\u7684\u8BA2\u9605\u8BA1\u5212\uFF0C\u7559\u7A7A\u8868\u793A\u4E0D\u9650\u5236\u8BA1\u5212",
          "empty": "\u6CA1\u6709\u627E\u5230\u5339\u914D\u7684\u8BA2\u9605"
        },
        "code": {
          "label": "\u81EA\u5B9A\u4E49\u4F18\u60E0\u7801",
          "placeholder": "\u81EA\u5B9A\u4E49\u4F18\u60E0\u7801\uFF0C\u7559\u7A7A\u5219\u81EA\u52A8\u751F\u6210",
          "description": "\u53EF\u4EE5\u81EA\u5B9A\u4E49\u4F18\u60E0\u7801\uFF0C\u7559\u7A7A\u5219\u7CFB\u7EDF\u81EA\u52A8\u751F\u6210"
        },
        "generateCount": {
          "label": "\u6279\u91CF\u751F\u6210\u6570\u91CF",
          "placeholder": "\u6279\u91CF\u751F\u6210\u4F18\u60E0\u7801\u6570\u91CF\uFF0C\u7559\u7A7A\u5219\u751F\u6210\u5355\u4E2A",
          "description": "\u6279\u91CF\u751F\u6210\u591A\u4E2A\u4F18\u60E0\u7801\uFF0C\u7559\u7A7A\u5219\u53EA\u751F\u6210\u5355\u4E2A\u4F18\u60E0\u7801"
        },
        "submit": {
          "saving": "\u4FDD\u5B58\u4E2D...",
          "save": "\u4FDD\u5B58"
        },
        "error": {
          "saveFailed": "\u4FDD\u5B58\u4F18\u60E0\u5238\u5931\u8D25"
        },
        "timeRange": {
          "quickSet": "\u5FEB\u901F\u8BBE\u7F6E",
          "presets": {
            "1week": "1\u5468",
            "2weeks": "2\u5468",
            "1month": "1\u4E2A\u6708",
            "3months": "3\u4E2A\u6708",
            "6months": "6\u4E2A\u6708",
            "1year": "1\u5E74"
          }
        }
      },
      "period": {
        "hourly": "\u5C0F\u65F6",
        "daily": "\u5929",
        "monthly": "\u6708",
        "quarterly": "\u5B63\u5EA6",
        "half_yearly": "\u534A\u5E74",
        "yearly": "\u5E74",
        "two_yearly": "\u4E24\u5E74",
        "three_yearly": "\u4E09\u5E74",
        "onetime": "\u4E00\u6B21\u6027",
        "reset_traffic": "\u91CD\u7F6E\u6D41\u91CF"
      }
    },
    "route": {
      "title": "\u8DEF\u7531\u7BA1\u7406",
      "description": "\u7BA1\u7406\u6240\u6709\u8DEF\u7531\u7EC4\uFF0C\u5305\u62EC\u6DFB\u52A0\u3001\u5220\u9664\u3001\u7F16\u8F91\u7B49\u64CD\u4F5C\u3002",
      "columns": {
        "id": "\u7EC4ID",
        "remarks": "\u5907\u6CE8",
        "action": "\u52A8\u4F5C",
        "actions": "\u64CD\u4F5C",
        "matchRules": "\u5339\u914D{{count}}\u6761\u89C4\u5219",
        "action_value": {
          "title": "\u52A8\u4F5C\u503C",
          "dns": "DNS: {{value}}",
          "proxy": "\u8F6C\u53D1 ({{value}})",
          "block": "\u963B\u6B62\u8BBF\u95EE",
          "direct": "\u76F4\u8FDE"
        }
      },
      "actions": {
        "dns": "\u6307\u5B9ADNS\u670D\u52A1\u5668\u8FDB\u884C\u89E3\u6790",
        "block": "\u7981\u6B62\u8BBF\u95EE",
        "direct": "\u76F4\u8FDE",
        "proxy": "\u8F6C\u53D1",
        "short": {
          "dns": "DNS",
          "block": "\u963B\u6B62",
          "direct": "\u76F4\u8FDE",
          "proxy": "\u8F6C\u53D1"
        }
      },
      "form": {
        "add": "\u6DFB\u52A0\u8DEF\u7531",
        "edit": "\u7F16\u8F91\u8DEF\u7531",
        "create": "\u521B\u5EFA\u8DEF\u7531",
        "remarks": "\u5907\u6CE8",
        "remarksPlaceholder": "\u8BF7\u8F93\u5165\u5907\u6CE8",
        "match": "\u5339\u914D\u89C4\u5219",
        "matchPlaceholder": "example.com\n*.example.com",
        "action": "\u52A8\u4F5C",
        "actionPlaceholder": "\u8BF7\u9009\u62E9\u52A8\u4F5C",
        "dns": "DNS\u670D\u52A1\u5668",
        "dnsPlaceholder": "\u8BF7\u8F93\u5165DNS\u670D\u52A1\u5668",
        "proxy": "\u8F6C\u53D1\u6807\u7B7E (Outbound Tag)",
        "proxyPlaceholder": "\u8BF7\u8F93\u5165\u8F6C\u53D1\u6807\u7B7E",
        "cancel": "\u53D6\u6D88",
        "submit": "\u63D0\u4EA4",
        "validation": {
          "remarks": "\u8BF7\u8F93\u5165\u6709\u6548\u7684\u5907\u6CE8"
        }
      },
      "toolbar": {
        "searchPlaceholder": "\u641C\u7D22\u8DEF\u7531...",
        "reset": "\u91CD\u7F6E"
      },
      "messages": {
        "deleteConfirm": "\u786E\u8BA4\u5220\u9664",
        "deleteDescription": "\u6B64\u64CD\u4F5C\u5C06\u6C38\u4E45\u5220\u9664\u8BE5\u8DEF\u7531\u7EC4\uFF0C\u5220\u9664\u540E\u65E0\u6CD5\u6062\u590D\u3002\u786E\u5B9A\u8981\u7EE7\u7EED\u5417\uFF1F",
        "deleteButton": "\u5220\u9664",
        "deleteSuccess": "\u5220\u9664\u6210\u529F",
        "createSuccess": "\u521B\u5EFA\u6210\u529F",
        "updateSuccess": "\u66F4\u65B0\u6210\u529F"
      }
    },
    "ticket": {
      "title": "\u5DE5\u5355\u7BA1\u7406",
      "description": "\u5728\u8FD9\u91CC\u53EF\u4EE5\u67E5\u770B\u7528\u6237\u5DE5\u5355\uFF0C\u5305\u62EC\u67E5\u770B\u3001\u56DE\u590D\u3001\u5173\u95ED\u7B49\u64CD\u4F5C\u3002",
      "columns": {
        "id": "\u5DE5\u5355\u53F7",
        "select": "\u9009\u62E9",
        "subject": "\u4E3B\u9898",
        "level": "\u4F18\u5148\u7EA7",
        "status": "\u72B6\u6001",
        "updated_at": "\u6700\u540E\u66F4\u65B0",
        "created_at": "\u521B\u5EFA\u65F6\u95F4",
        "actions": "\u64CD\u4F5C"
      },
      "status": {
        "closed": "\u5DF2\u5173\u95ED",
        "replied": "\u5DF2\u56DE\u590D",
        "pending": "\u5F85\u56DE\u590D",
        "processing": "\u5904\u7406\u4E2D",
        "unreplied": "\u672A\u56DE\u590D"
      },
      "level": {
        "low": "\u4F4E\u4F18\u5148",
        "medium": "\u4E2D\u4F18\u5148",
        "high": "\u9AD8\u4F18\u5148"
      },
      "filter": {
        "placeholder": "\u641C\u7D22{field}...",
        "no_results": "\u672A\u627E\u5230\u7ED3\u679C",
        "selected": "\u5DF2\u9009\u62E9 {count} \u9879",
        "clear": "\u6E05\u9664\u7B5B\u9009"
      },
      "actions": {
        "reply_success": "\u5DF2\u56DE\u590D",
        "view_details": "\u67E5\u770B\u8BE6\u60C5",
        "close_ticket": "\u5173\u95ED\u5DE5\u5355",
        "close_confirm_title": "\u786E\u8BA4\u5173\u95ED\u5DE5\u5355",
        "close_confirm_description": "\u786E\u5B9A\u8981\u5173\u95ED\u8FD9\u4E2A\u5DE5\u5355\u5417\uFF1F\u5173\u95ED\u540E\u4F1A\u79FB\u5165\u5DF2\u5173\u95ED\u5217\u8868\uFF0C\u4F46\u4ECD\u53EF\u7EE7\u7EED\u56DE\u590D\u3002",
        "close_confirm_button": "\u786E\u8BA4\u5173\u95ED",
        "close_success": "\u5DE5\u5355\u5DF2\u5173\u95ED",
        "view_ticket": "\u67E5\u770B\u5DE5\u5355",
        "bulk_close": "\u6279\u91CF\u5173\u95ED ({{count}})",
        "bulk_close_confirm_title": "\u786E\u8BA4\u6279\u91CF\u5173\u95ED\u5DE5\u5355",
        "bulk_close_confirm_description": "\u786E\u5B9A\u8981\u5173\u95ED\u9009\u4E2D\u7684 {{count}} \u4E2A\u5DE5\u5355\u5417\uFF1F\u5173\u95ED\u540E\u4ECD\u53EF\u7EE7\u7EED\u56DE\u590D\u3002",
        "bulk_close_confirm_button": "\u786E\u8BA4\u5173\u95ED",
        "bulk_close_success": "\u5DF2\u5173\u95ED {{count}} \u4E2A\u5DE5\u5355"
      },
      "detail": {
        "no_messages": "\u6682\u65E0\u6D88\u606F\u8BB0\u5F55",
        "created_at": "\u521B\u5EFA\u4E8E",
        "sender_admin": "\u7BA1\u7406\u5458",
        "sender_user": "\u7528\u6237",
        "user_info": "\u7528\u6237\u4FE1\u606F",
        "traffic_records": "\u6D41\u91CF\u8BB0\u5F55",
        "order_records": "\u8BA2\u5355\u8BB0\u5F55",
        "message": {
          "edit": "\u4FEE\u6539",
          "recall": "\u64A4\u56DE",
          "save": "\u4FDD\u5B58",
          "cancel": "\u53D6\u6D88",
          "edited": "\u5DF2\u7F16\u8F91",
          "recalled": "\u8BE5\u6D88\u606F\u5DF2\u64A4\u56DE",
          "edit_success": "\u6D88\u606F\u5DF2\u4FEE\u6539",
          "recall_success": "\u6D88\u606F\u5DF2\u64A4\u56DE",
          "recall_confirm_title": "\u786E\u8BA4\u64A4\u56DE\u6D88\u606F",
          "recall_confirm_description": "\u64A4\u56DE\u540E\u7528\u6237\u5C06\u65E0\u6CD5\u518D\u770B\u5230\u8FD9\u6761\u6D88\u606F\uFF0C\u4E14\u65E0\u6CD5\u6062\u590D\u3002",
          "recall_confirm_button": "\u786E\u8BA4\u64A4\u56DE"
        },
        "input": {
          "closed_reply_placeholder": "\u5DE5\u5355\u5DF2\u5173\u95ED\uFF0C\u4ECD\u53EF\u7EE7\u7EED\u56DE\u590D...",
          "closed_hint": "\u8BE5\u5DE5\u5355\u5DF2\u5173\u95ED\uFF0C\u4F60\u4ECD\u53EF\u4EE5\u7EE7\u7EED\u56DE\u590D\uFF0C\u65B0\u7684\u6D88\u606F\u4F1A\u8FFD\u52A0\u5230\u5F53\u524D\u5DE5\u5355\u3002",
          "reply_placeholder": "\u8F93\u5165\u56DE\u590D\u5185\u5BB9...",
          "sending": "\u53D1\u9001\u4E2D...",
          "send": "\u53D1\u9001",
          "shortcut_hint": "Enter \u53D1\u9001 \xB7 Shift + Enter \u6362\u884C"
        }
      },
      "list": {
        "title": "\u5DE5\u5355\u5217\u8868",
        "search_placeholder": "\u641C\u7D22\u5DE5\u5355\u6807\u9898\u6216\u7528\u6237\u90AE\u7BB1",
        "no_tickets": "\u6682\u65E0\u5DE5\u5355",
        "no_open_tickets": "\u6682\u65E0\u5904\u7406\u4E2D\u5DE5\u5355",
        "no_closed_tickets": "\u6682\u65E0\u5DF2\u5173\u95ED\u5DE5\u5355",
        "no_search_results": "\u672A\u627E\u5230\u5339\u914D\u7684\u5DE5\u5355",
        "collapse": "\u6536\u8D77\u5217\u8868",
        "expand": "\u5C55\u5F00\u5217\u8868"
      }
    },
    "withdrawal": {
      "title": "\u63D0\u73B0\u7BA1\u7406",
      "description": "\u7BA1\u7406\u7528\u6237\u7684\u4F63\u91D1\u63D0\u73B0\u7533\u8BF7\uFF0C\u652F\u6301\u786E\u8BA4\u63D0\u73B0\u548C\u62D2\u7EDD\u63D0\u73B0\u64CD\u4F5C\u3002",
      "columns": {
        "id": "\u63D0\u73B0\u5355\u53F7",
        "user": "\u7528\u6237",
        "method": "\u63D0\u73B0\u65B9\u5F0F",
        "account": "\u63D0\u73B0\u8D26\u53F7",
        "amount": "\u91D1\u989D",
        "status": "\u72B6\u6001",
        "created_at": "\u521B\u5EFA\u65F6\u95F4",
        "actions": "\u64CD\u4F5C"
      },
      "status": {
        "pending": "\u5F85\u5904\u7406",
        "confirmed": "\u5DF2\u786E\u8BA4",
        "closed": "\u5DF2\u62D2\u7EDD"
      },
      "filter": {
        "all": "\u5168\u90E8"
      },
      "actions": {
        "view": "\u67E5\u770B\u8BE6\u60C5",
        "confirm": "\u786E\u8BA4\u63D0\u73B0",
        "close": "\u5173\u95ED\u63D0\u73B0",
        "user_info": "\u7528\u6237\u4FE1\u606F",
        "confirm_title": "\u786E\u8BA4\u63D0\u73B0",
        "confirm_description": "\u786E\u8BA4\u540E\u5C06\u5173\u95ED\u8BE5\u63D0\u73B0\u5355\uFF0C\u8BF7\u786E\u4FDD\u6B3E\u9879\u5DF2\u5904\u7406\u3002",
        "confirm_button": "\u786E\u8BA4\u63D0\u73B0",
        "confirm_success": "\u63D0\u73B0\u5DF2\u786E\u8BA4",
        "close_confirm_title": "\u5173\u95ED\u63D0\u73B0",
        "close_confirm_description": "\u5173\u95ED\u540E\u5C06\u628A\u4F63\u91D1\u8FD4\u8FD8\u7ED9\u7528\u6237\u8D26\u6237\u3002",
        "close_confirm_button": "\u786E\u8BA4\u5173\u95ED",
        "close_success": "\u63D0\u73B0\u5DF2\u5173\u95ED\uFF0C\u4F63\u91D1\u5DF2\u8FD4\u8FD8",
        "reply_success": "\u56DE\u590D\u6210\u529F",
        "confirm_desc": "\u786E\u8BA4\u540E\u5C06\u5173\u95ED\u8BE5\u63D0\u73B0\u5355\uFF0C\u8BF7\u786E\u4FDD\u6B3E\u9879\u5DF2\u5904\u7406\u3002",
        "close_desc": "\u5173\u95ED\u540E\u5C06\u628A\u4F63\u91D1\u8FD4\u8FD8\u7ED9\u7528\u6237\u8D26\u6237\uFF0C\u662F\u5426\u7EE7\u7EED\uFF1F"
      },
      "list": {
        "search_placeholder": "\u641C\u7D22\u63D0\u73B0\u8D26\u53F7...",
        "no_withdrawals": "\u6682\u65E0\u63D0\u73B0\u8BB0\u5F55",
        "no_open_withdrawals": "\u6682\u65E0\u5F85\u5904\u7406\u63D0\u73B0",
        "title": "\u63D0\u73B0\u5217\u8868",
        "collapse": "\u6536\u8D77\u5217\u8868",
        "expand": "\u5C55\u5F00\u5217\u8868"
      },
      "detail": {
        "created_at": "\u521B\u5EFA\u65F6\u95F4",
        "user_info": "\u7528\u6237\u4FE1\u606F",
        "traffic_records": "\u6D41\u91CF\u8BB0\u5F55",
        "order_records": "\u8BA2\u5355\u8BB0\u5F55",
        "no_messages": "\u6682\u65E0\u804A\u5929\u8BB0\u5F55",
        "operator": "\u64CD\u4F5C\u5458",
        "remark": "\u5907\u6CE8",
        "remark_placeholder": "\u8F93\u5165\u5907\u6CE8\uFF08\u53EF\u9009\uFF09",
        "sender_user": "\u7528\u6237",
        "sender_admin": "\u7BA1\u7406\u5458",
        "input": {
          "closed_hint": "\u8BE5\u63D0\u73B0\u5355\u5DF2\u5904\u7406\uFF0C\u65E0\u6CD5\u7EE7\u7EED\u56DE\u590D",
          "reply_placeholder": "\u8F93\u5165\u6D88\u606F...",
          "closed_reply_placeholder": "\u8BE5\u63D0\u73B0\u5355\u5DF2\u5904\u7406",
          "send": "\u53D1\u9001",
          "sending": "\u53D1\u9001\u4E2D...",
          "shortcut_hint": "\u56DE\u8F66\u53D1\u9001\uFF0CShift+\u56DE\u8F66\u6362\u884C"
        }
      }
    },
    "machine": {
      "title": "\u670D\u52A1\u5668\u7BA1\u7406",
      "description": "\u7528\u4E8E\u67E5\u770B\u670D\u52A1\u5668\u5065\u5EB7\u3001\u8D1F\u8F7D\u4E0E\u627F\u8F7D\u8282\u70B9\uFF0C\u5E76\u4ECE\u8FD0\u7EF4\u89C6\u89D2\u7BA1\u7406 Fboard-Node \u670D\u52A1\u3002",
      "columns": {
        "id": "ID",
        "name": "\u670D\u52A1\u5668\u540D\u79F0",
        "status": "\u72B6\u6001",
        "nodes": "\u8282\u70B9\u6570",
        "nodesHosted": "\u8282\u70B9\u6570",
        "nodesIdle": "\u6682\u65E0\u627F\u8F7D",
        "load": "\u8D1F\u8F7D",
        "lastSeen": "\u6700\u540E\u5FC3\u8DF3",
        "version": "\u7248\u672C",
        "actions": "\u64CD\u4F5C",
        "online": "\u5728\u7EBF",
        "offline": "\u79BB\u7EBF",
        "inactive": "\u5DF2\u7981\u7528",
        "noData": "\u6682\u65E0\u8D1F\u8F7D\u6570\u636E",
        "cpu": "CPU",
        "memory": "\u5185\u5B58",
        "disk": "\u78C1\u76D8",
        "never": "\u4ECE\u672A\u4E0A\u62A5",
        "lastReport": "\u8D1F\u8F7D\u4E0A\u62A5",
        "kernel": "\u5185\u6838",
        "kernelRunning": "\u8FD0\u884C\u4E2D",
        "kernelStopped": "\u5DF2\u505C\u6B62",
        "kernelPartial": "\u90E8\u5206\u8FD0\u884C",
        "kernelIdle": "\u65E0\u8282\u70B9",
        "kernelUnknown": "\u672A\u77E5",
        "kernelDetail": "{{running}}/{{total}} \u8FD0\u884C"
      },
      "toolbar": {
        "search": "\u641C\u7D22\u670D\u52A1\u5668\u540D\u79F0\u6216\u5907\u6CE8...",
        "status": "\u72B6\u6001",
        "status_all": "\u5168\u90E8\u72B6\u6001",
        "status_online": "\u5728\u7EBF",
        "status_offline": "\u79BB\u7EBF",
        "status_inactive": "\u5DF2\u7981\u7528",
        "status_high_load": "\u9AD8\u8D1F\u8F7D",
        "nodes": "\u8282\u70B9",
        "with_nodes": "\u5DF2\u627F\u8F7D\u8282\u70B9",
        "idle_nodes": "\u7A7A\u95F2\u670D\u52A1\u5668",
        "high_load": "\u9AD8\u8D1F\u8F7D",
        "online_ratio": "\u5728\u7EBF",
        "high_load_count": "\u9AD8\u8D1F\u8F7D",
        "tip": "\u9002\u5408\u96C6\u4E2D\u67E5\u770B\u670D\u52A1\u5668\u5728\u7EBF\u60C5\u51B5\u3001\u627F\u8F7D\u8282\u70B9\u6570\u91CF\u4E0E\u8D44\u6E90\u538B\u529B\u3002",
        "reset": "\u91CD\u7F6E",
        "nodesHosted": "\u8282\u70B9",
        "nodesIdle": "\u7A7A\u95F2"
      },
      "operations": {
        "upgrade": "\u5347\u7EA7 Fboard-Node",
        "ops": "\u5185\u6838\u8FD0\u7EF4",
        "start": "\u542F\u52A8\u5185\u6838",
        "stop": "\u505C\u6B62\u5185\u6838",
        "reload": "\u91CD\u8F7D\u5185\u6838",
        "restart": "\u91CD\u542F\u5185\u6838",
        "upgradeTitle": "\u786E\u8BA4\u5347\u7EA7\u670D\u52A1\u5668",
        "upgradeDescription": "\u5C06\u5347\u7EA7\u670D\u52A1\u5668\u201C{{name}}\u201D\u4E0A\u7684 Fboard-Node \u670D\u52A1\u3002\u5347\u7EA7\u8FC7\u7A0B\u4E2D\u8BE5\u670D\u52A1\u5668\u4E0A\u7684\u670D\u52A1\u53EF\u80FD\u77ED\u6682\u4E2D\u65AD\u3002",
        "startTitle": "\u786E\u8BA4\u542F\u52A8\u5185\u6838",
        "startDescription": "\u5C06\u542F\u52A8\u670D\u52A1\u5668\u201C{{name}}\u201D\u4E0A\u6240\u6709\u8282\u70B9\u7684\u5185\u5D4C xray \u5185\u6838\u3002fboard-node \u8FDB\u7A0B\u4E0E WebSocket \u4FDD\u6301\u8FD0\u884C\u3002",
        "stopTitle": "\u786E\u8BA4\u505C\u6B62\u5185\u6838",
        "stopDescription": "\u5C06\u505C\u6B62\u670D\u52A1\u5668\u201C{{name}}\u201D\u4E0A\u6240\u6709\u8282\u70B9\u7684\u5185\u5D4C xray \u5185\u6838\uFF0C\u8BE5\u670D\u52A1\u5668\u4E0A\u7684\u4EE3\u7406\u5C06\u4E0D\u53EF\u7528\uFF0C\u76F4\u5230\u518D\u6B21\u542F\u52A8\u3002\u8FDB\u7A0B\u4E0E WebSocket \u4FDD\u6301\u8FD0\u884C\u3002",
        "reloadTitle": "\u786E\u8BA4\u91CD\u8F7D\u5185\u6838",
        "reloadDescription": "\u5C06\u91CD\u8F7D\u670D\u52A1\u5668\u201C{{name}}\u201D\u4E0A\u6240\u6709\u8282\u70B9\u7684\u5185\u5D4C xray \u5185\u6838\u914D\u7F6E\uFF1B\u8FDB\u7A0B\u4E0E WebSocket \u4FDD\u6301\u8FD0\u884C\u3002",
        "restartTitle": "\u786E\u8BA4\u91CD\u542F\u5185\u6838",
        "restartDescription": "\u5C06\u5F3A\u5236\u91CD\u5EFA\u670D\u52A1\u5668\u201C{{name}}\u201D\u4E0A\u6240\u6709\u8282\u70B9\u7684\u5185\u5D4C xray \u5185\u6838\uFF0C\u4EE3\u7406\u4F1A\u77ED\u6682\u4E2D\u65AD\u3002fboard-node \u8FDB\u7A0B\u4E0E WebSocket \u4FDD\u6301\u8FD0\u884C\u3002",
        "upgradeSubmitted": "\u670D\u52A1\u5668\u201C{{name}}\u201D\u7684\u5347\u7EA7\u4EFB\u52A1\u5DF2\u63D0\u4EA4",
        "upgradeSuccess": "\u670D\u52A1\u5668\u201C{{name}}\u201D\u5347\u7EA7\u6210\u529F",
        "upgradeFailed": "\u670D\u52A1\u5668\u201C{{name}}\u201D\u5347\u7EA7\u5931\u8D25\uFF1A{{detail}}",
        "upgradePending": "\u670D\u52A1\u5668\u201C{{name}}\u201D\u5347\u7EA7\u4EFB\u52A1\u5DF2\u4E0B\u53D1\uFF0C\u6682\u672A\u6536\u5230\u7ED3\u679C",
        "startSubmitted": "\u670D\u52A1\u5668\u201C{{name}}\u201D\u7684\u5185\u6838\u542F\u52A8\u4EFB\u52A1\u5DF2\u63D0\u4EA4",
        "stopSubmitted": "\u670D\u52A1\u5668\u201C{{name}}\u201D\u7684\u5185\u6838\u505C\u6B62\u4EFB\u52A1\u5DF2\u63D0\u4EA4",
        "reloadSubmitted": "\u670D\u52A1\u5668\u201C{{name}}\u201D\u7684\u5185\u6838\u91CD\u8F7D\u4EFB\u52A1\u5DF2\u63D0\u4EA4",
        "restartSubmitted": "\u670D\u52A1\u5668\u201C{{name}}\u201D\u7684\u5185\u6838\u91CD\u542F\u4EFB\u52A1\u5DF2\u63D0\u4EA4",
        "batchUpgrade": "\u4E00\u952E\u5347\u7EA7\u670D\u52A1\u5668",
        "batchUpgradeTitle": "\u786E\u8BA4\u6279\u91CF\u5347\u7EA7\u670D\u52A1\u5668",
        "batchUpgradeDescription": "\u5C06\u5BF9\u6240\u6709\u5728\u7EBF\u4E14\u542F\u7528\u7684\u670D\u52A1\u5668\u5404\u63D0\u4EA4\u4E00\u6B21 Fboard-Node \u5347\u7EA7\u4EFB\u52A1\uFF0C\u4E0D\u4F9D\u8D56\u670D\u52A1\u5668\u627F\u8F7D\u7684\u8282\u70B9\u6570\u91CF\u3002",
        "batchUpgradeSubmitted": "\u5DF2\u63D0\u4EA4 {{submitted}} \u53F0\u670D\u52A1\u5668\uFF1B\u8DF3\u8FC7 {{inactive}} \u53F0\u7981\u7528\u3001{{offline}} \u53F0\u79BB\u7EBF\u670D\u52A1\u5668"
      },
      "overview": {
        "total": "\u670D\u52A1\u5668\u603B\u6570",
        "total_hint": "\u5171\u627F\u8F7D {{count}} \u4E2A\u8282\u70B9",
        "online": "\u5728\u7EBF\u670D\u52A1\u5668",
        "online_hint": "\u6700\u8FD1 5 \u5206\u949F\u5185\u6B63\u5E38\u5FC3\u8DF3",
        "offline": "\u79BB\u7EBF/\u5931\u8054",
        "offline_hint": "\u9700\u8981\u68C0\u67E5\u5FC3\u8DF3\u6216\u8282\u70B9\u4EE3\u7406",
        "high_load": "\u9AD8\u8D1F\u8F7D",
        "high_load_hint": "CPU\u3001\u5185\u5B58\u6216\u78C1\u76D8\u63A5\u8FD1\u9608\u503C",
        "nodes_suffix": "\u8282\u70B9",
        "attention": "\u9700\u5173\u6CE8",
        "stable": "\u7A33\u5B9A",
        "needs_review": "\u5EFA\u8BAE\u68C0\u67E5",
        "normal": "\u6B63\u5E38"
      },
      "form": {
        "add": "\u6DFB\u52A0\u670D\u52A1\u5668",
        "create": "\u65B0\u5EFA\u670D\u52A1\u5668",
        "edit": "\u7F16\u8F91\u670D\u52A1\u5668",
        "createDescription": "\u5F53\u4F60\u5E0C\u671B\u4E00\u53F0\u670D\u52A1\u5668\u627F\u8F7D\u591A\u4E2A\u8282\u70B9\u65F6\uFF0C\u518D\u521B\u5EFA\u670D\u52A1\u5668\u8BB0\u5F55\u3002",
        "editDescription": "\u4FEE\u6539\u670D\u52A1\u5668\u540D\u79F0\u3001\u5907\u6CE8\u6216\u542F\u7528\u72B6\u6001\u3002",
        "name": "\u670D\u52A1\u5668\u540D\u79F0",
        "namePlaceholder": "\u4F8B\u5982 HK-01",
        "nameError": "\u8BF7\u8F93\u5165\u670D\u52A1\u5668\u540D\u79F0",
        "notes": "\u5907\u6CE8",
        "notesPlaceholder": "\u5173\u4E8E\u6B64\u670D\u52A1\u5668\u7684\u53EF\u9009\u5907\u6CE8",
        "isActive": "\u542F\u7528\u670D\u52A1\u5668",
        "isActiveDescription": "\u7981\u7528\u540E Fboard-Node \u5C06\u4E0D\u518D\u4F7F\u7528\u6B64\u670D\u52A1\u5668\u3002",
        "cancel": "\u53D6\u6D88",
        "submit": "\u63D0\u4EA4",
        "update": "\u66F4\u65B0"
      },
      "token": {
        "title": "\u670D\u52A1\u5668 Token",
        "description": "\u6B64 Token \u7528\u4E8E Fboard-Node \u5411\u9762\u677F\u8BA4\u8BC1\uFF0C\u8BF7\u59A5\u5584\u4FDD\u7BA1\u3002",
        "show": "\u67E5\u770B Token",
        "hide": "\u9690\u85CF Token",
        "reset": "\u91CD\u7F6E Token",
        "resetConfirm": "\u786E\u8BA4\u91CD\u7F6E Token\uFF1F",
        "resetDescription": "\u65E7 Token \u5C06\u7ACB\u5373\u5931\u6548\uFF0CFboard-Node \u9700\u8981\u91CD\u65B0\u914D\u7F6E\u65B0 Token\u3002",
        "copy": "\u590D\u5236",
        "copied": "Token \u5DF2\u590D\u5236\u5230\u526A\u8D34\u677F",
        "copiedInline": "\u5DF2\u590D\u5236!",
        "copyFailed": "\u590D\u5236\u5931\u8D25\uFF0C\u8BF7\u624B\u52A8\u590D\u5236",
        "autoHide": "{{time}} \u540E\u81EA\u52A8\u9690\u85CF",
        "resetSuccess": "Token \u5DF2\u91CD\u7F6E",
        "createdHint": "Token \u5DF2\u751F\u6210\uFF0C\u540E\u7EED\u53EF\u5728\u670D\u52A1\u5668\u8BE6\u60C5\u9875\u4E2D\u67E5\u770B\u3002"
      },
      "install": {
        "title": "\u5B89\u88C5 Fboard-Node",
        "description": "\u5728\u76EE\u6807\u670D\u52A1\u5668\u4E0A\u6267\u884C\u6B64\u547D\u4EE4\uFF0C\u5373\u53EF\u7528 machine mode \u5B89\u88C5 Fboard-Node \u5E76\u63A5\u5165\u5F53\u524D\u670D\u52A1\u5668\u8BB0\u5F55\u3002",
        "copy": "\u590D\u5236\u5B89\u88C5\u547D\u4EE4",
        "copied": "\u5B89\u88C5\u547D\u4EE4\u5DF2\u590D\u5236",
        "copiedInline": "\u5DF2\u590D\u5236!",
        "copyFailed": "\u590D\u5236\u5931\u8D25",
        "loading": "\u6B63\u5728\u751F\u6210\u547D\u4EE4...",
        "hint": "\u9700\u8981 root \u6216 sudo \u6743\u9650\uFF0C\u4E14\u76EE\u6807\u670D\u52A1\u5668\u9700\u4E3A\u652F\u6301 systemd \u7684 Linux\u3002"
      },
      "logs": {
        "title": "\u8FD0\u884C\u65E5\u5FD7",
        "description": "\u6765\u81EA Fboard-Node \u8FDB\u7A0B\u5185\u5B58\u4E2D\u7684\u6700\u8FD1\u65E5\u5FD7\uFF08\u6700\u591A\u7EA6 1000 \u884C\uFF09\u3002",
        "refresh": "\u5237\u65B0\u65E5\u5FD7",
        "copy": "\u590D\u5236\u65E5\u5FD7",
        "copied": "\u65E5\u5FD7\u5DF2\u590D\u5236",
        "loading": "\u6B63\u5728\u62C9\u53D6\u65E5\u5FD7...",
        "empty": "\u6682\u65E0\u65E5\u5FD7",
        "offline": "\u670D\u52A1\u5668\u79BB\u7EBF\uFF0C\u65E0\u6CD5\u62C9\u53D6\u5B9E\u65F6\u65E5\u5FD7",
        "stale": "\u663E\u793A\u7F13\u5B58\u65E5\u5FD7\uFF08\u53EF\u80FD\u4E0D\u662F\u6700\u65B0\uFF09",
        "timeout": "\u7B49\u5F85\u8282\u70B9\u54CD\u5E94\u8D85\u65F6",
        "updatedAt": "\u66F4\u65B0\u4E8E {{time}}",
        "lineCount": "{{count}} \u884C",
        "autoScroll": "\u81EA\u52A8\u6EDA\u52A8\u5230\u5E95\u90E8",
        "fetchFailed": "\u62C9\u53D6\u65E5\u5FD7\u5931\u8D25"
      },
      "detail": {
        "title": "\u670D\u52A1\u5668\u8BE6\u60C5",
        "info": "\u670D\u52A1\u5668\u4FE1\u606F",
        "associatedNodes": "\u5173\u8054\u8282\u70B9",
        "noNodes": "\u6682\u65E0\u7ED1\u5B9A\u8282\u70B9\u3002",
        "nodeId": "ID",
        "nodeName": "\u540D\u79F0",
        "nodeType": "\u7C7B\u578B",
        "nodeHost": "\u5730\u5740",
        "nodePort": "\u7AEF\u53E3",
        "nodeShow": "\u53EF\u89C1",
        "nodeEnabled": "\u5DF2\u6FC0\u6D3B",
        "loadTrend": "\u8D1F\u8F7D\u8D8B\u52BF",
        "networkTrend": "\u7F51\u7EDC\u901F\u7387",
        "noHistory": "\u6682\u65E0\u5386\u53F2\u8D1F\u8F7D\u6570\u636E",
        "openNodeManage": "\u524D\u5F80\u8282\u70B9\u7BA1\u7406",
        "addNodeToServer": "\u65B0\u589E\u8282\u70B9\u5230\u6B64\u670D\u52A1\u5668",
        "nodeCount": "{{count}} \u4E2A\u8282\u70B9",
        "nodeEnabledCount": "{{count}} \u4E2A\u5DF2\u6FC0\u6D3B",
        "toggleEnabledError": "\u5207\u6362\u8282\u70B9\u72B6\u6001\u5931\u8D25",
        "bindExistingButton": "\u5173\u8054\u5DF2\u6709\u8282\u70B9",
        "bindExistingTitle": "\u5173\u8054\u5DF2\u6709\u8282\u70B9",
        "bindExistingDescription": "\u9009\u62E9\u8981\u5173\u8054\u5230\u300C{{name}}\u300D\u7684\u8282\u70B9",
        "bindSearchPlaceholder": "\u641C\u7D22\u8282\u70B9\u540D\u79F0\u3001\u5730\u5740\u3001\u7C7B\u578B...",
        "bindTypeAll": "\u5168\u90E8\u7C7B\u578B",
        "noUnboundNodes": "\u6CA1\u6709\u672A\u7ED1\u5B9A\u7684\u8282\u70B9",
        "noSearchResults": "\u6CA1\u6709\u5339\u914D\u7684\u8282\u70B9",
        "selectAll": "\u5168\u9009\uFF08\u5171 {{count}} \u4E2A\uFF09",
        "selectedCount": "\u5DF2\u9009 {{count}} \u4E2A",
        "bindConfirm": "\u5173\u8054 {{count}} \u4E2A\u8282\u70B9",
        "binding": "\u5173\u8054\u4E2D...",
        "bindSuccess": "\u6210\u529F\u5C06 {{count}} \u4E2A\u8282\u70B9\u5173\u8054\u5230\u300C{{name}}\u300D",
        "bindFailed": "\u5173\u8054\u5931\u8D25",
        "unbindNode": "\u53D6\u6D88\u5173\u8054",
        "unbindSuccess": "\u5DF2\u53D6\u6D88\u300C{{name}}\u300D\u7684\u5173\u8054",
        "unbindFailed": "\u53D6\u6D88\u5173\u8054\u5931\u8D25",
        "unbindConfirmTitle": "\u786E\u8BA4\u53D6\u6D88\u5173\u8054",
        "unbindConfirmDescription": "\u5C06\u628A\u8282\u70B9\u300C{{name}}\u300D\u4ECE\u5F53\u524D\u670D\u52A1\u5668\u4E0A\u89E3\u7ED1\uFF08\u8282\u70B9\u672C\u8EAB\u4E0D\u4F1A\u88AB\u5220\u9664\uFF09\u3002",
        "cancel": "\u53D6\u6D88"
      },
      "messages": {
        "createSuccess": "\u670D\u52A1\u5668\u521B\u5EFA\u6210\u529F",
        "updateSuccess": "\u670D\u52A1\u5668\u66F4\u65B0\u6210\u529F",
        "deleteConfirm": "\u786E\u8BA4\u5220\u9664\u670D\u52A1\u5668\uFF1F",
        "deleteDescription": "\u5173\u8054\u8282\u70B9\u5C06\u81EA\u52A8\u89E3\u7ED1\uFF08\u4E0D\u4F1A\u88AB\u5220\u9664\uFF09\uFF0C\u6B64\u64CD\u4F5C\u4E0D\u53EF\u64A4\u9500\u3002",
        "deleteButton": "\u5220\u9664",
        "deleteSuccess": "\u670D\u52A1\u5668\u5220\u9664\u6210\u529F",
        "deleteFailed": "\u5220\u9664\u670D\u52A1\u5668\u5931\u8D25",
        "saveFailed": "\u4FDD\u5B58\u670D\u52A1\u5668\u5931\u8D25",
        "tokenFetchFailed": "\u83B7\u53D6\u4EE4\u724C\u5931\u8D25",
        "tokenResetFailed": "\u91CD\u7F6E\u4EE4\u724C\u5931\u8D25"
      },
      "nodeForm": {
        "machineId": "\u7ED1\u5B9A\u670D\u52A1\u5668",
        "machineIdPlaceholder": "\u9009\u62E9\u4E00\u53F0\u670D\u52A1\u5668\uFF08\u53EF\u9009\uFF09",
        "machineIdNone": "\u72EC\u7ACB\u90E8\u7F72",
        "enabled": "\u5728\u670D\u52A1\u5668\u4E0A\u6FC0\u6D3B",
        "enabledDescription": "\u8282\u70B9\u662F\u5426\u5728\u6240\u9009\u670D\u52A1\u5668\u4E0A\u542F\u7528\u8FD0\u884C"
      },
      "nodesStatus": {
        "toggleHint": "\u5728\u8282\u70B9\u7BA1\u7406\u4E2D\u4FEE\u6539\u72B6\u6001"
      }
    },
    "search": {
      "placeholder": "\u641C\u7D22\u83DC\u5355\u548C\u529F\u80FD...",
      "title": "\u83DC\u5355\u5BFC\u822A",
      "noResults": "\u672A\u627E\u5230\u7ED3\u679C",
      "shortcut": {
        "label": "\u641C\u7D22",
        "key": "\u2318K"
      }
    },
    "knowledge": {
      "title": "\u77E5\u8BC6\u5E93\u7BA1\u7406",
      "description": "\u5728\u8FD9\u91CC\u53EF\u4EE5\u914D\u7F6E\u77E5\u8BC6\u5E93\uFF0C\u5305\u62EC\u6DFB\u52A0\u3001\u5220\u9664\u3001\u7F16\u8F91\u7B49\u64CD\u4F5C\u3002",
      "columns": {
        "id": "ID",
        "status": "\u72B6\u6001",
        "title": "\u6807\u9898",
        "category": "\u5206\u7C7B",
        "actions": "\u64CD\u4F5C"
      },
      "form": {
        "add": "\u6DFB\u52A0\u77E5\u8BC6",
        "edit": "\u7F16\u8F91\u77E5\u8BC6",
        "title": "\u6807\u9898",
        "titlePlaceholder": "\u8BF7\u8F93\u5165\u77E5\u8BC6\u6807\u9898",
        "category": "\u5206\u7C7B",
        "categoryPlaceholder": "\u8BF7\u8F93\u5165\u5206\u7C7B\uFF0C\u5206\u7C7B\u5C06\u4F1A\u81EA\u52A8\u5F52\u7C7B",
        "language": "\u8BED\u8A00",
        "languagePlaceholder": "\u8BF7\u9009\u62E9\u8BED\u8A00",
        "content": "\u5185\u5BB9",
        "show": "\u663E\u793A",
        "cancel": "\u53D6\u6D88",
        "submit": "\u63D0\u4EA4"
      },
      "languages": {
        "en-US": "English",
        "ja-JP": "\u65E5\u672C\u8A9E",
        "ko-KR": "\uD55C\uAD6D\uC5B4",
        "vi-VN": "Ti\u1EBFng Vi\u1EC7t",
        "zh-CN": "\u7B80\u4F53\u4E2D\u6587",
        "zh-TW": "\u7E41\u9AD4\u4E2D\u6587",
        "ru-RU": "\u0420\u0443\u0441\u0441\u043A\u0438\u0439"
      },
      "messages": {
        "deleteConfirm": "\u786E\u8BA4\u5220\u9664",
        "deleteDescription": "\u6B64\u64CD\u4F5C\u5C06\u6C38\u4E45\u5220\u9664\u8BE5\u77E5\u8BC6\u5E93\u8BB0\u5F55\uFF0C\u5220\u9664\u540E\u65E0\u6CD5\u6062\u590D\u3002\u786E\u5B9A\u8981\u7EE7\u7EED\u5417\uFF1F",
        "deleteButton": "\u5220\u9664",
        "operationSuccess": "\u64CD\u4F5C\u6210\u529F",
        "loadError": "\u52A0\u8F7D\u5931\u8D25"
      },
      "toolbar": {
        "searchPlaceholder": "\u641C\u7D22\u77E5\u8BC6...",
        "reset": "\u91CD\u7F6E",
        "sortModeHint": "\u62D6\u62FD\u77E5\u8BC6\u6761\u76EE\u8FDB\u884C\u6392\u5E8F\uFF0C\u5B8C\u6210\u540E\u70B9\u51FB\u4FDD\u5B58",
        "editSort": "\u7F16\u8F91\u6392\u5E8F",
        "saveSort": "\u4FDD\u5B58\u6392\u5E8F",
        "allCategories": "\u5168\u90E8\u5206\u7C7B"
      }
    },
    "common": {
      "all": "\u5168\u90E8",
      "clear": "\u6E05\u9664",
      "selectAll": "\u5168\u9009",
      "loading": "\u52A0\u8F7D\u4E2D...",
      "error": "\u9519\u8BEF",
      "success": "\u6210\u529F",
      "save": "\u4FDD\u5B58",
      "cancel": "\u53D6\u6D88",
      "confirm": "\u786E\u8BA4",
      "close": "\u5173\u95ED",
      "delete": {
        "success": "\u5220\u9664\u6210\u529F",
        "failed": "\u5220\u9664\u5931\u8D25"
      },
      "edit": "\u7F16\u8F91",
      "view": "\u67E5\u770B",
      "toggleNavigation": "\u5207\u6362\u5BFC\u822A",
      "toggleSidebar": "\u5207\u6362\u4FA7\u8FB9\u680F",
      "search": "\u641C\u7D22...",
      "noMatch": "\u65E0\u5339\u914D\u9009\u9879",
      "selectField": "\u9009\u62E9{{name}}",
      "inputField": "\u8F93\u5165{{name}}",
      "pageNotImplemented": "\u8BE5\u9875\u9762\u5F85\u5B9E\u73B0",
      "theme": {
        "label": "\u4E3B\u9898",
        "light": "\u6D45\u8272",
        "dark": "\u6DF1\u8272",
        "system": "\u8DDF\u968F\u7CFB\u7EDF"
      },
      "user": "\u7528\u6237",
      "defaultEmail": "user@example.com",
      "settings": "\u8BBE\u7F6E",
      "logout": "\u9000\u51FA\u767B\u5F55",
      "copy": {
        "success": "\u590D\u5236\u6210\u529F",
        "failed": "\u590D\u5236\u5931\u8D25",
        "error": "\u590D\u5236\u5931\u8D25",
        "errorLog": "\u590D\u5236\u5230\u526A\u8D34\u677F\u65F6\u51FA\u9519"
      },
      "submit": "\u63D0\u4EA4",
      "saving": "\u4FDD\u5B58\u4E2D...",
      "table": {
        "noData": "\u6682\u65E0\u6570\u636E",
        "pagination": {
          "selected": "\u5DF2\u9009\u62E9 {{selected}} \u9879\uFF0C\u5171 {{total}} \u9879",
          "itemsPerPage": "\u6BCF\u9875",
          "page": "\u7B2C",
          "pageOf": "\u7B2C {{page}}/{{total}} \u9875",
          "range": "\u7B2C {{from}}\u2013{{to}} \u6761\uFF0C\u5171 {{total}} \u6761",
          "firstPage": "\u8DF3\u8F6C\u5230\u7B2C\u4E00\u9875",
          "previousPage": "\u4E0A\u4E00\u9875",
          "nextPage": "\u4E0B\u4E00\u9875",
          "lastPage": "\u8DF3\u8F6C\u5230\u6700\u540E\u4E00\u9875"
        },
        "viewOptions": {
          "button": "\u663E\u793A\u5217",
          "label": "\u5207\u6362\u663E\u793A\u5217"
        }
      },
      "update": {
        "title": "\u7CFB\u7EDF\u66F4\u65B0",
        "newVersion": "\u53D1\u73B0\u65B0\u7248\u672C",
        "currentVersion": "\u5F53\u524D\u7248\u672C",
        "latestVersion": "\u6700\u65B0\u7248\u672C",
        "updateLater": "\u7A0D\u540E\u66F4\u65B0",
        "updateNow": "\u7ACB\u5373\u66F4\u65B0",
        "updating": "\u66F4\u65B0\u4E2D...",
        "updateSuccess": "\u66F4\u65B0\u6210\u529F\uFF0C\u7CFB\u7EDF\u5C06\u5728\u7A0D\u540E\u81EA\u52A8\u91CD\u542F",
        "updateFailed": "\u66F4\u65B0\u5931\u8D25\uFF0C\u8BF7\u7A0D\u540E\u91CD\u8BD5"
      },
      "time": {
        "day": "\u5929",
        "hour": "\u5C0F\u65F6"
      },
      "reset": "\u91CD\u7F6E",
      "export": "\u5BFC\u51FA",
      "currency": {
        "yuan": "\u5143"
      },
      "http": {
        "notLoggedIn": "\u672A\u767B\u5F55",
        "unknownError": "\u672A\u77E5\u9519\u8BEF",
        "loginExpired": "\u767B\u5F55\u5DF2\u8FC7\u671F",
        "loginExpiredRelogin": "\u767B\u5F55\u5DF2\u8FC7\u671F\uFF0C\u8BF7\u91CD\u65B0\u767B\u5F55",
        "unauthorized": "\u672A\u6388\u6743\uFF0C\u8BF7\u91CD\u65B0\u767B\u5F55",
        "invalidCredentials": "\u90AE\u7BB1\u6216\u5BC6\u7801\u9519\u8BEF",
        "invalidData": "\u63D0\u4EA4\u7684\u6570\u636E\u6709\u8BEF\uFF0C\u8BF7\u68C0\u67E5\u8F93\u5165",
        "requestFailed": "\u8BF7\u6C42\u5931\u8D25",
        "networkError": "\u7F51\u7EDC\u9519\u8BEF",
        "noPermission": "\u6CA1\u6709\u6743\u9650",
        "notFound": "\u8D44\u6E90\u6216\u63A5\u53E3\u4E0D\u5B58\u5728",
        "unknownException": "\u672A\u77E5\u5F02\u5E38",
        "success": "\u64CD\u4F5C\u6210\u529F"
      },
      "add": "\u6DFB\u52A0",
      "refresh": "\u5237\u65B0",
      "sort": {
        "edit": "\u7F16\u8F91\u6392\u5E8F",
        "done": "\u5B8C\u6210\u6392\u5E8F"
      },
      "actions": "\u64CD\u4F5C",
      "start": "\u8D77",
      "end": "\u6B62"
    },
    "plugin": {
      "title": "\u63D2\u4EF6\u7BA1\u7406",
      "description": "\u7BA1\u7406\u548C\u914D\u7F6E\u7CFB\u7EDF\u63D2\u4EF6",
      "search": {
        "placeholder": "\u641C\u7D22\u63D2\u4EF6\u540D\u79F0\u6216\u63CF\u8FF0..."
      },
      "type": {
        "placeholder": "\u9009\u62E9\u63D2\u4EF6\u7C7B\u578B",
        "all": "\u5168\u90E8\u7C7B\u578B"
      },
      "tabs": {
        "all": "\u6240\u6709\u63D2\u4EF6",
        "installed": "\u5DF2\u5B89\u88C5",
        "available": "\u53EF\u7528"
      },
      "status": {
        "enabled": "\u5DF2\u542F\u7528",
        "disabled": "\u5DF2\u7981\u7528",
        "not_installed": "\u672A\u5B89\u88C5",
        "protected": "\u53D7\u4FDD\u62A4",
        "filter_placeholder": "\u5B89\u88C5\u72B6\u6001",
        "all": "\u5168\u90E8\u72B6\u6001",
        "installed": "\u5DF2\u5B89\u88C5",
        "available": "\u53EF\u5B89\u88C5"
      },
      "button": {
        "install": "\u5B89\u88C5",
        "upgrade": "\u5347\u7EA7",
        "config": "\u914D\u7F6E",
        "enable": "\u542F\u7528",
        "disable": "\u7981\u7528",
        "uninstall": "\u5378\u8F7D",
        "readme": "\u67E5\u770B\u6587\u6863",
        "menuDemo": "\u83DC\u5355\u6F14\u793A"
      },
      "upload": {
        "button": "\u4E0A\u4F20\u63D2\u4EF6",
        "title": "\u4E0A\u4F20\u63D2\u4EF6",
        "description": "\u4E0A\u4F20\u63D2\u4EF6\u5305 (.zip)",
        "dragText": "\u62D6\u62FD\u63D2\u4EF6\u5305\u5230\u6B64\u5904\uFF0C\u6216",
        "clickText": "\u6D4F\u89C8",
        "supportText": "\u4EC5\u652F\u6301 .zip \u683C\u5F0F\u6587\u4EF6",
        "uploading": "\u4E0A\u4F20\u4E2D...",
        "error": {
          "format": "\u4EC5\u652F\u6301 .zip \u683C\u5F0F\u6587\u4EF6"
        }
      },
      "delete": {
        "title": "\u5220\u9664\u63D2\u4EF6",
        "description": "\u786E\u5B9A\u8981\u5220\u9664\u6B64\u63D2\u4EF6\u5417\uFF1F\u6B64\u64CD\u4F5C\u65E0\u6CD5\u64A4\u9500\u3002",
        "button": "\u5220\u9664"
      },
      "uninstall": {
        "title": "\u5378\u8F7D\u63D2\u4EF6",
        "description": "\u786E\u5B9A\u8981\u5378\u8F7D\u6B64\u63D2\u4EF6\u5417\uFF1F\u5378\u8F7D\u540E\u63D2\u4EF6\u6570\u636E\u5C06\u88AB\u6E05\u9664\u3002",
        "button": "\u5378\u8F7D"
      },
      "upgrade": {
        "title": "\u5347\u7EA7\u63D2\u4EF6",
        "description": "\u786E\u5B9A\u8981\u5347\u7EA7\u6B64\u63D2\u4EF6\u5417\uFF1F\u5347\u7EA7\u8FC7\u7A0B\u4E2D\u63D2\u4EF6\u5C06\u6682\u65F6\u4E0D\u53EF\u7528\u3002",
        "button": "\u5347\u7EA7"
      },
      "config": {
        "title": "\u914D\u7F6E",
        "description": "\u4FEE\u6539\u63D2\u4EF6\u914D\u7F6E",
        "save": "\u4FDD\u5B58",
        "cancel": "\u53D6\u6D88",
        "noConfigs": "\u8BE5\u63D2\u4EF6\u6CA1\u6709\u53EF\u914D\u7F6E\u7684\u9009\u9879",
        "actions": "\u64CD\u4F5C",
        "selectPlan": "\u8BF7\u9009\u62E9\u5957\u9910",
        "noPlan": "\u4E0D\u9009\u62E9\u5957\u9910"
      },
      "readme": {
        "title": "\u63D2\u4EF6\u6587\u6863",
        "empty": "\u6682\u65E0\u6587\u6863"
      },
      "author": "\u4F5C\u8005",
      "messages": {
        "installSuccess": "\u63D2\u4EF6\u5B89\u88C5\u6210\u529F",
        "installError": "\u63D2\u4EF6\u5B89\u88C5\u5931\u8D25",
        "upgradeSuccess": "\u63D2\u4EF6\u5347\u7EA7\u6210\u529F",
        "upgradeError": "\u63D2\u4EF6\u5347\u7EA7\u5931\u8D25",
        "uninstallSuccess": "\u63D2\u4EF6\u5378\u8F7D\u6210\u529F",
        "uninstallError": "\u63D2\u4EF6\u5378\u8F7D\u5931\u8D25",
        "enableSuccess": "\u63D2\u4EF6\u542F\u7528\u6210\u529F",
        "enableError": "\u63D2\u4EF6\u542F\u7528\u5931\u8D25",
        "disableSuccess": "\u63D2\u4EF6\u7981\u7528\u6210\u529F",
        "disableError": "\u63D2\u4EF6\u7981\u7528\u5931\u8D25",
        "configLoadError": "\u52A0\u8F7D\u63D2\u4EF6\u914D\u7F6E\u5931\u8D25",
        "configSaveSuccess": "\u914D\u7F6E\u4FDD\u5B58\u6210\u529F",
        "configSaveError": "\u914D\u7F6E\u4FDD\u5B58\u5931\u8D25",
        "uploadSuccess": "\u63D2\u4EF6\u4E0A\u4F20\u6210\u529F",
        "uploadError": "\u63D2\u4EF6\u4E0A\u4F20\u5931\u8D25",
        "deleteSuccess": "\u63D2\u4EF6\u5220\u9664\u6210\u529F",
        "deleteError": "\u63D2\u4EF6\u5220\u9664\u5931\u8D25",
        "actionSuccess": "\u6267\u884C\u6210\u529F",
        "actionError": "\u6267\u884C\u5931\u8D25",
        "actionLabel": "\u52A8\u4F5C",
        "actionSuccessWithLabel": "{{label}} \u6267\u884C\u6210\u529F",
        "actionErrorWithLabel": "{{label}} \u6267\u884C\u5931\u8D25",
        "invalidJson": "{{field}} \u4E0D\u662F\u6709\u6548\u7684 JSON"
      },
      "staticFiles": {
        "title": "HTML \u9759\u6001\u6587\u4EF6",
        "previewTitle": "\u63D2\u4EF6 HTML \u9884\u89C8",
        "backToList": "\u8FD4\u56DE\u5217\u8868",
        "openInNewTab": "\u65B0\u6807\u7B7E\u9875\u6253\u5F00",
        "empty": "\u6682\u65E0\u9759\u6001\u6587\u4EF6"
      },
      "noPlugins": "\u6682\u65E0\u63D2\u4EF6",
      "toolbar": {
        "search": "\u641C\u7D22\u63D2\u4EF6..."
      }
    },
    "dashboard": {
      "title": "\u4EEA\u8868\u76D8",
      "stats": {
        "newUsers": "\u65B0\u7528\u6237",
        "totalScore": "\u603B\u79EF\u5206",
        "monthlyUpload": "\u6708\u4E0A\u4F20",
        "vsLastMonth": "\u5BF9\u6BD4\u4E0A\u6708",
        "vsYesterday": "\u5BF9\u6BD4\u6628\u65E5",
        "todayIncome": "\u4ECA\u65E5\u6536\u5165",
        "monthlyIncome": "\u6708\u6536\u5165",
        "totalIncome": "\u603B\u6536\u5165",
        "totalUsers": "\u603B\u7528\u6237",
        "activeUsers": "\u6D3B\u8DC3\u7528\u6237: {{count}}",
        "totalOrders": "\u603B\u8BA2\u5355",
        "revenue": "\u6536\u5165",
        "todayRegistered": "\u4ECA\u65E5\u6CE8\u518C",
        "monthlyRegistered": "\u6708\u6CE8\u518C",
        "onlineUsers": "\u5728\u7EBF\u7528\u6237",
        "pendingTickets": "\u5F85\u5904\u7406\u5DE5\u5355",
        "hasPendingTickets": "\u6709\u5DE5\u5355\u9700\u8981\u5904\u7406",
        "noPendingTickets": "\u65E0\u5F85\u5904\u7406\u5DE5\u5355",
        "pendingCommission": "\u5F85\u5904\u7406\u4F63\u91D1",
        "hasPendingCommission": "\u6709\u4F63\u91D1\u9700\u8981\u786E\u8BA4",
        "noPendingCommission": "\u65E0\u5F85\u5904\u7406\u4F63\u91D1",
        "monthlyNewUsers": "\u6708\u65B0\u589E\u7528\u6237",
        "monthlyDownload": "\u6708\u4E0B\u8F7D",
        "todayTraffic": "\u4ECA\u65E5: {{value}}",
        "activeUserTrend": "\u6D3B\u8DC3\u7528\u6237\u8D8B\u52BF",
        "realtimeUsers": "\u5B9E\u65F6\u7528\u6237",
        "todayPeak": "\u4ECA\u65E5\u5CF0\u503C",
        "vsLastWeek": "\u5BF9\u6BD4\u4E0A\u5468"
      },
      "trafficRank": {
        "nodeTrafficRank": "\u8282\u70B9\u6D41\u91CF\u6392\u884C",
        "userTrafficRank": "\u7528\u6237\u6D41\u91CF\u6392\u884C",
        "today": "\u4ECA\u5929",
        "last7days": "\u6700\u8FD17\u5929",
        "last30days": "\u6700\u8FD130\u5929",
        "customRange": "\u81EA\u5B9A\u4E49\u8303\u56F4",
        "selectTimeRange": "\u9009\u62E9\u65F6\u95F4\u8303\u56F4",
        "selectDateRange": "\u9009\u62E9\u65E5\u671F\u8303\u56F4",
        "currentTraffic": "\u5F53\u524D\u6D41\u91CF",
        "previousTraffic": "\u4E0A\u671F\u6D41\u91CF",
        "changeRate": "\u53D8\u5316\u7387",
        "recordTime": "\u8BB0\u5F55\u65F6\u95F4"
      },
      "overview": {
        "title": "\u6536\u5165\u6982\u89C8",
        "thisMonth": "\u672C\u6708",
        "lastMonth": "\u4E0A\u6708",
        "to": "\u81F3",
        "selectTimeRange": "\u9009\u62E9\u8303\u56F4",
        "selectDate": "\u9009\u62E9\u65E5\u671F",
        "last7Days": "\u6700\u8FD17\u5929",
        "last30Days": "\u6700\u8FD130\u5929",
        "last90Days": "\u6700\u8FD190\u5929",
        "last180Days": "\u6700\u8FD1180\u5929",
        "lastYear": "\u6700\u8FD1\u4E00\u5E74",
        "customRange": "\u81EA\u5B9A\u4E49\u8303\u56F4",
        "amount": "\u91D1\u989D",
        "count": "\u6570\u91CF",
        "transactions": "{{count}} \u7B14\u4EA4\u6613",
        "orderAmount": "\u8BA2\u5355\u91D1\u989D",
        "commissionAmount": "\u4F63\u91D1\u91D1\u989D",
        "orderCount": "\u8BA2\u5355\u6570\u91CF",
        "commissionCount": "\u4F63\u91D1\u6570\u91CF",
        "totalIncome": "\u603B\u6536\u5165",
        "totalCommission": "\u603B\u4F63\u91D1",
        "totalTransactions": "\u5171 {{count}} \u7B14\u4EA4\u6613",
        "avgOrderAmount": "\u5E73\u5747\u8BA2\u5355\u91D1\u989D:",
        "commissionRate": "\u4F63\u91D1\u6BD4\u4F8B:"
      },
      "queue": {
        "title": "\u961F\u5217\u72B6\u6001",
        "metrics": {
          "pending": "\u5F85\u5904\u7406 {{count}}",
          "maxWait": "\u6700\u957F\u7B49\u5F85 {{time}}",
          "backlog": "\u79EF\u538B",
          "processes": "\u8FDB\u7A0B",
          "pendingLabel": "\u5F85\u5904\u7406",
          "wait": "\u7B49\u5F85"
        },
        "jobDetails": "\u4F5C\u4E1A\u8BE6\u60C5",
        "workload": "\u961F\u5217\u8D1F\u8F7D",
        "workloadCount": "\u5171 {{count}} \u4E2A\u961F\u5217",
        "status": {
          "description": "Horizon \u8FD0\u884C\u72B6\u6001\u4E0E\u5404\u961F\u5217\u79EF\u538B",
          "running": "\u8FD0\u884C\u72B6\u6001",
          "normal": "\u6B63\u5E38",
          "abnormal": "\u5F02\u5E38",
          "waitTime": "\u5F53\u524D\u7B49\u5F85\u65F6\u95F4\uFF1A{{seconds}} \u79D2",
          "pending": "\u7B49\u5F85\u4E2D",
          "processing": "\u5904\u7406\u4E2D",
          "completed": "\u5DF2\u5B8C\u6210",
          "failed": "\u5931\u8D25",
          "cancelled": "\u5DF2\u53D6\u6D88"
        },
        "details": {
          "description": "\u961F\u5217\u5904\u7406\u8BE6\u7EC6\u4FE1\u606F",
          "recentJobs": "\u8FD1\u671F\u4EFB\u52A1\u6570",
          "statisticsPeriod": "\u7EDF\u8BA1\u65F6\u95F4\u8303\u56F4\uFF1A{{hours}} \u5C0F\u65F6",
          "jobsPerMinute": "\u6BCF\u5206\u949F\u5904\u7406\u91CF",
          "maxThroughput": "\u6700\u9AD8\u541E\u5410\u91CF\uFF1A{{value}}",
          "failedJobs7Days": "7\u65E5\u62A5\u9519\u6570\u91CF",
          "retentionPeriod": "\u4FDD\u7559 {{hours}} \u5C0F\u65F6",
          "longestRunningQueue": "\u6700\u957F\u8FD0\u884C\u961F\u5217",
          "activeProcesses": "\u6D3B\u8DC3\u8FDB\u7A0B",
          "id": "\u4F5C\u4E1AID",
          "type": "\u4F5C\u4E1A\u7C7B\u578B",
          "status": "\u72B6\u6001",
          "progress": "\u8FDB\u5EA6",
          "createdAt": "\u521B\u5EFA\u65F6\u95F4",
          "updatedAt": "\u66F4\u65B0\u65F6\u95F4",
          "error": "\u9519\u8BEF\u4FE1\u606F",
          "data": "\u4F5C\u4E1A\u6570\u636E",
          "result": "\u7ED3\u679C",
          "duration": "\u8017\u65F6",
          "attempts": "\u91CD\u8BD5\u6B21\u6570",
          "nextRetry": "\u4E0B\u6B21\u91CD\u8BD5",
          "failedJobsDetailTitle": "\u5931\u8D25\u4EFB\u52A1\u8BE6\u60C5",
          "viewFailedJobs": "\u67E5\u770B\u62A5\u9519\u8BE6\u60C5",
          "jobDetailTitle": "\u4EFB\u52A1\u8BE6\u7EC6\u4FE1\u606F",
          "time": "\u65F6\u95F4",
          "queue": "\u961F\u5217",
          "name": "\u4EFB\u52A1\u540D\u79F0",
          "exception": "\u5F02\u5E38\u4FE1\u606F",
          "noException": "\u6682\u65E0\u5F02\u5E38\u4FE1\u606F",
          "noFailedJobs": "\u6682\u65E0\u5931\u8D25\u4EFB\u52A1",
          "connection": "\u8FDE\u63A5\u7C7B\u578B",
          "payload": "\u4EFB\u52A1\u6570\u636E",
          "viewDetail": "\u67E5\u770B\u8BE6\u60C5",
          "action": "\u64CD\u4F5C"
        },
        "actions": {
          "retry": "\u91CD\u8BD5",
          "cancel": "\u53D6\u6D88",
          "delete": "\u5220\u9664",
          "viewDetails": "\u67E5\u770B\u8BE6\u60C5"
        },
        "empty": "\u961F\u5217\u4E2D\u6682\u65E0\u4F5C\u4E1A",
        "loading": "\u6B63\u5728\u52A0\u8F7D\u961F\u5217\u72B6\u6001...",
        "error": "\u52A0\u8F7D\u961F\u5217\u72B6\u6001\u5931\u8D25"
      },
      "common": {
        "refresh": "\u5237\u65B0",
        "close": "\u5173\u95ED",
        "pagination": "\u7B2C {{current}}/{{total}} \u9875\uFF0C\u5171 {{count}} \u6761"
      },
      "search": {
        "loading": "\u641C\u7D22\u4E2D...",
        "noResults": "\u672A\u627E\u5230\u7ED3\u679C",
        "placeholder": "\u641C\u7D22\u83DC\u5355\u548C\u529F\u80FD...",
        "title": "\u83DC\u5355\u5BFC\u822A"
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
      "title": "\u8BA2\u5355\u7BA1\u7406",
      "description": "\u5728\u8FD9\u91CC\u53EF\u4EE5\u67E5\u770B\u7528\u6237\u8BA2\u5355\uFF0C\u5305\u62EC\u5206\u914D\u3001\u67E5\u770B\u3001\u5220\u9664\u7B49\u64CD\u4F5C\u3002",
      "table": {
        "columns": {
          "tradeNo": "\u8BA2\u5355\u53F7",
          "select": "\u9009\u62E9",
          "type": "\u7C7B\u578B",
          "user": "\u7528\u6237",
          "plan": "\u8BA2\u9605\u8BA1\u5212",
          "period": "\u5468\u671F",
          "amount": "\u652F\u4ED8\u91D1\u989D",
          "status": "\u8BA2\u5355\u72B6\u6001",
          "commission": "\u4F63\u91D1",
          "commissionStatus": "\u4F63\u91D1\u72B6\u6001",
          "createdAt": "\u521B\u5EFA\u65F6\u95F4",
          "actions": "\u64CD\u4F5C"
        }
      },
      "type": {
        "NEW": "\u65B0\u8D2D",
        "RENEWAL": "\u7EED\u8D39",
        "UPGRADE": "\u5347\u7EA7",
        "RESET_FLOW": "\u6D41\u91CF\u91CD\u7F6E",
        "DEPOSIT": "\u4F59\u989D\u5145\u503C"
      },
      "period": {
        "hour_price": "\u65F6\u4ED8",
        "day_price": "\u65E5\u4ED8",
        "month_price": "\u6708\u4ED8",
        "quarter_price": "\u5B63\u4ED8",
        "half_year_price": "\u534A\u5E74\u4ED8",
        "year_price": "\u5E74\u4ED8",
        "two_year_price": "\u4E24\u5E74\u4ED8",
        "three_year_price": "\u4E09\u5E74\u4ED8",
        "onetime_price": "\u4E00\u6B21\u6027",
        "reset_price": "\u6D41\u91CF\u91CD\u7F6E\u5305",
        "deposit": "\u4F59\u989D\u5145\u503C",
        "hourly": "\u5C0F\u65F6",
        "daily": "\u5929",
        "monthly": "\u6708",
        "quarterly": "\u5B63\u5EA6",
        "half_yearly": "\u534A\u5E74",
        "yearly": "\u5E74",
        "two_yearly": "\u4E24\u5E74",
        "three_yearly": "\u4E09\u5E74",
        "onetime": "\u4E00\u6B21\u6027",
        "reset_traffic": "\u91CD\u7F6E\u6D41\u91CF"
      },
      "status": {
        "PENDING": "\u5F85\u652F\u4ED8",
        "PROCESSING": "\u5F00\u901A\u4E2D",
        "CANCELLED": "\u5DF2\u53D6\u6D88",
        "COMPLETED": "\u5DF2\u5B8C\u6210",
        "DISCOUNTED": "\u5DF2\u6298\u62B5",
        "tooltip": "\u6807\u8BB0\u4E3A[\u5DF2\u652F\u4ED8]\u540E\u5C06\u4F1A\u7531\u7CFB\u7EDF\u8FDB\u884C\u5F00\u901A\u540E\u5E76\u5B8C\u6210"
      },
      "commission": {
        "PENDING": "\u5F85\u786E\u8BA4",
        "PROCESSING": "\u53D1\u653E\u4E2D",
        "VALID": "\u6709\u6548",
        "INVALID": "\u65E0\u6548"
      },
      "filter": {
        "allTypes": "\u5168\u90E8\u7C7B\u578B",
        "allPeriods": "\u5168\u90E8\u5468\u671F",
        "allStatuses": "\u5168\u90E8\u72B6\u6001",
        "allCommissions": "\u5168\u90E8\u4F63\u91D1\u72B6\u6001",
        "userId": "\u7528\u6237 ID",
        "clear": "\u6E05\u9664",
        "clearAll": "\u6E05\u9664\u7B5B\u9009"
      },
      "actions": {
        "view": "\u67E5\u770B\u8BE6\u60C5",
        "markAsPaid": "\u6807\u8BB0\u4E3A\u5DF2\u652F\u4ED8",
        "cancel": "\u53D6\u6D88\u8BA2\u5355",
        "issue": "\u53D1\u653E\u4F63\u91D1",
        "invalid": "\u65E0\u6548\u4F63\u91D1",
        "openMenu": "\u6253\u5F00\u83DC\u5355",
        "reset": "\u91CD\u7F6E",
        "copyTradeNo": "\u590D\u5236\u8BA2\u5355\u53F7",
        "batchConfirmCommission": "\u6279\u91CF\u786E\u8BA4\u4F63\u91D1 ({{count}})",
        "batchConfirmCommissionConfirmTitle": "\u786E\u8BA4\u6279\u91CF\u53D1\u653E\u4F63\u91D1",
        "batchConfirmCommissionConfirmButton": "\u786E\u8BA4\u53D1\u653E"
      },
      "search": {
        "placeholder": "\u641C\u7D22\u8BA2\u5355\u53F7..."
      },
      "dialog": {
        "title": "\u8BA2\u5355\u4FE1\u606F",
        "basicInfo": "\u57FA\u672C\u4FE1\u606F",
        "amountInfo": "\u91D1\u989D\u4FE1\u606F",
        "timeInfo": "\u65F6\u95F4\u4FE1\u606F",
        "commissionInfo": "\u4F63\u91D1\u4FE1\u606F",
        "commissionStatusActive": "\u6709\u6548",
        "addOrder": "\u6DFB\u52A0\u8BA2\u5355",
        "assignOrder": "\u8BA2\u5355\u5206\u914D",
        "fields": {
          "userEmail": "\u7528\u6237\u90AE\u7BB1",
          "userPhone": "\u7528\u6237\u624B\u673A\u53F7",
          "orderPeriod": "\u8BA2\u5355\u5468\u671F",
          "subscriptionPlan": "\u8BA2\u9605\u8BA1\u5212",
          "callbackNo": "\u56DE\u8C03\u5355\u53F7",
          "paymentAmount": "\u652F\u4ED8\u91D1\u989D",
          "balancePayment": "\u4F59\u989D\u652F\u4ED8",
          "discountAmount": "\u4F18\u60E0\u91D1\u989D",
          "refundAmount": "\u9000\u56DE\u91D1\u989D",
          "deductionAmount": "\u6298\u62B5\u91D1\u989D",
          "createdAt": "\u521B\u5EFA\u65F6\u95F4",
          "updatedAt": "\u66F4\u65B0\u65F6\u95F4",
          "commissionStatus": "\u4F63\u91D1\u72B6\u6001",
          "commissionAmount": "\u4F63\u91D1\u91D1\u989D",
          "actualCommissionAmount": "\u5B9E\u9645\u4F63\u91D1",
          "inviteUser": "\u9080\u8BF7\u4EBA",
          "inviteUserId": "\u9080\u8BF7\u4EBAID"
        },
        "placeholders": {
          "email": "\u8BF7\u8F93\u5165\u7528\u6237\u90AE\u7BB1",
          "plan": "\u8BF7\u9009\u62E9\u8BA2\u9605\u8BA1\u5212",
          "period": "\u8BF7\u9009\u62E9\u8D2D\u4E70\u65F6\u957F",
          "amount": "\u8BF7\u8F93\u5165\u9700\u8981\u652F\u4ED8\u7684\u91D1\u989D"
        },
        "actions": {
          "cancel": "\u53D6\u6D88",
          "confirm": "\u786E\u5B9A"
        },
        "messages": {
          "addSuccess": "\u6DFB\u52A0\u6210\u529F",
          "addOrder": "\u6DFB\u52A0\u8BA2\u5355"
        }
      },
      "messages": {
        "addSuccess": "\u6DFB\u52A0\u6210\u529F",
        "markPaidSuccess": "\u5DF2\u6807\u8BB0\u4E3A\u5DF2\u652F\u4ED8",
        "cancelSuccess": "\u8BA2\u5355\u5DF2\u53D6\u6D88",
        "cancelConfirm": "\u786E\u8BA4\u53D6\u6D88\u8BE5\u8BA2\u5355\uFF1F",
        "commissionIssueSuccess": "\u5DF2\u53D1\u653E\u4F63\u91D1",
        "commissionInvalidSuccess": "\u5DF2\u6807\u8BB0\u4E3A\u65E0\u6548\u4F63\u91D1",
        "commissionInvalidConfirm": "\u786E\u8BA4\u5C06\u8BE5\u4F63\u91D1\u6807\u8BB0\u4E3A\u65E0\u6548\uFF1F",
        "bulkCommissionConfirmConfirm": "\u786E\u5B9A\u8981\u786E\u8BA4\u9009\u4E2D\u7684 {{count}} \u4E2A\u8BA2\u5355\u4F63\u91D1\u5417\uFF1F\u786E\u8BA4\u540E\u5C06\u8FDB\u5165\u53D1\u653E\u6D41\u7A0B\u3002",
        "bulkCommissionConfirmSuccess": "\u5DF2\u786E\u8BA4 {{count}} \u7B14\u4F63\u91D1"
      }
    }
  };
  window.FBOARD_TRANSLATIONS = window.FBOARD_TRANSLATIONS ?? {};
  window.FBOARD_TRANSLATIONS["zh-CN"] = translations;
})();
