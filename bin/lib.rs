mod components {
    mod schemas {
        #[derive(Debug, Serialize, Deserialize)]
        struct NullableSimpleUser {
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            email: Option<String>,
            login: String,
            id: i64,
            node_id: String,
            avatar_url: String,
            gravatar_id: Option<String>,
            url: String,
            html_url: String,
            followers_url: String,
            following_url: String,
            gists_url: String,
            starred_url: String,
            subscriptions_url: String,
            organizations_url: String,
            repos_url: String,
            events_url: String,
            received_events_url: String,
            type: String,
            site_admin: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            starred_at: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct IntegrationPermissions {
            #[serde(skip_serializing_if = "Option::is_none")]
            issues: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            checks: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            metadata: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            contents: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            deployments: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Integration {
            id: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            slug: Option<String>,
            node_id: String,
            owner: crate::components::schemas::NullableSimpleUser,
            name: String,
            description: Option<String>,
            external_url: String,
            html_url: String,
            created_at: String,
            updated_at: String,
            permissions: IntegrationPermissions,
            events: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            installations_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            client_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            client_secret: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook_secret: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pem: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BasicError {
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            documentation_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            status: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ValidationErrorSimple {
            message: String,
            documentation_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            errors: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct WebhookConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<crate::components::schemas::WebhookConfigUrl>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content_type: Option<crate::components::schemas::WebhookConfigContentType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            secret: Option<crate::components::schemas::WebhookConfigSecret>,
            #[serde(skip_serializing_if = "Option::is_none")]
            insecure_ssl: Option<crate::components::schemas::WebhookConfigInsecureSsl>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct HookDeliveryItem {
            id: i64,
            guid: String,
            delivered_at: String,
            redelivery: bool,
            duration: i64,
            status: String,
            status_code: i64,
            event: String,
            action: Option<String>,
            installation_id: Option<i64>,
            repository_id: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimError {
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            documentation_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            detail: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            status: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            scimType: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            schemas: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        enum ValidationErrorErrorsValueOneOf {
            String(String),
            i64(i64),
            Vec(Vec<String>),
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ValidationErrorErrors {
            #[serde(skip_serializing_if = "Option::is_none")]
            resource: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            field: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            code: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            index: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<ValidationErrorErrorsValueOneOf>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ValidationError {
            message: String,
            documentation_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            errors: Option<Vec<ValidationErrorErrors>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct HookDeliveryRequestHeaders;

        #[derive(Debug, Serialize, Deserialize)]
        struct HookDeliveryRequestPayload;

        #[derive(Debug, Serialize, Deserialize)]
        struct HookDeliveryRequest {
            headers: Option<HookDeliveryRequestHeaders>,
            payload: Option<HookDeliveryRequestPayload>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct HookDeliveryResponseHeaders;

        #[derive(Debug, Serialize, Deserialize)]
        struct HookDeliveryResponse {
            headers: Option<HookDeliveryResponseHeaders>,
            payload: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct HookDelivery {
            id: i64,
            guid: String,
            delivered_at: String,
            redelivery: bool,
            duration: i64,
            status: String,
            status_code: i64,
            event: String,
            action: Option<String>,
            installation_id: Option<i64>,
            repository_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            request: HookDeliveryRequest,
            response: HookDeliveryResponse,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct SimpleUser {
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            email: Option<String>,
            login: String,
            id: i64,
            node_id: String,
            avatar_url: String,
            gravatar_id: Option<String>,
            url: String,
            html_url: String,
            followers_url: String,
            following_url: String,
            gists_url: String,
            starred_url: String,
            subscriptions_url: String,
            organizations_url: String,
            repos_url: String,
            events_url: String,
            received_events_url: String,
            type: String,
            site_admin: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            starred_at: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Enterprise {
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            html_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            website_url: Option<String>,
            id: i64,
            node_id: String,
            name: String,
            slug: String,
            created_at: Option<String>,
            updated_at: Option<String>,
            avatar_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct AppPermissions {
            #[serde(skip_serializing_if = "Option::is_none")]
            actions: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            administration: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            checks: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content_references: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            contents: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            deployments: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            environments: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            issues: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            metadata: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            packages: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pages: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pull_requests: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repository_hooks: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repository_projects: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            secret_scanning_alerts: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            secrets: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            security_events: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            single_file: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            statuses: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            vulnerability_alerts: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            workflows: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            members: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            organization_administration: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            organization_hooks: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            organization_plan: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            organization_projects: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            organization_packages: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            organization_secrets: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            organization_self_hosted_runners: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            organization_user_blocking: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            team_discussions: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Installation {
            id: i64,
            account: Option<>,
            repository_selection: String,
            access_tokens_url: String,
            repositories_url: String,
            html_url: String,
            app_id: i64,
            target_id: i64,
            target_type: String,
            permissions: crate::components::schemas::AppPermissions,
            events: Vec<String>,
            created_at: String,
            updated_at: String,
            single_file_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_multiple_single_files: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            single_file_paths: Option<Vec<String>>,
            app_slug: String,
            suspended_by: crate::components::schemas::NullableSimpleUser,
            suspended_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            contact_email: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableLicenseSimple {
            key: String,
            name: String,
            url: Option<String>,
            spdx_id: Option<String>,
            node_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RepositoryPermissions {
            admin: bool,
            pull: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            triage: Option<bool>,
            push: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            maintain: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RepositoryTemplateRepositoryOwner {
            #[serde(skip_serializing_if = "Option::is_none")]
            login: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            avatar_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            gravatar_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            followers_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            following_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            gists_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            starred_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subscriptions_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            organizations_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repos_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            events_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            received_events_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            site_admin: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RepositoryTemplateRepositoryPermissions {
            #[serde(skip_serializing_if = "Option::is_none")]
            admin: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            maintain: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            push: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            triage: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pull: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RepositoryTemplateRepository {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            full_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            owner: Option<RepositoryTemplateRepositoryOwner>,
            #[serde(skip_serializing_if = "Option::is_none")]
            private: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            fork: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            archive_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            assignees_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            blobs_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            branches_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            collaborators_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            comments_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            commits_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            compare_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            contents_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            contributors_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            deployments_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            downloads_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            events_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            forks_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            git_commits_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            git_refs_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            git_tags_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            git_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            issue_comment_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            issue_events_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            issues_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            keys_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            labels_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            languages_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            merges_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            milestones_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            notifications_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pulls_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            releases_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ssh_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            stargazers_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            statuses_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subscribers_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subscription_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            tags_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            teams_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            trees_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            clone_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            mirror_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            hooks_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            svn_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            homepage: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            language: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            forks_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            stargazers_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            watchers_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            size: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            default_branch: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            open_issues_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_template: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            topics: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_issues: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_projects: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_wiki: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_pages: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_downloads: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            archived: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            disabled: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            visibility: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pushed_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            updated_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<RepositoryTemplateRepositoryPermissions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_rebase_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            temp_clone_token: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_squash_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_auto_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            delete_branch_on_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_merge_commit: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subscribers_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            network_count: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Repository {
            id: i64,
            node_id: String,
            name: String,
            full_name: String,
            license: crate::components::schemas::NullableLicenseSimple,
            #[serde(skip_serializing_if = "Option::is_none")]
            organization: Option<crate::components::schemas::NullableSimpleUser>,
            forks: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<RepositoryPermissions>,
            owner: crate::components::schemas::SimpleUser,
            private: bool,
            html_url: String,
            description: Option<String>,
            fork: bool,
            url: String,
            archive_url: String,
            assignees_url: String,
            blobs_url: String,
            branches_url: String,
            collaborators_url: String,
            comments_url: String,
            commits_url: String,
            compare_url: String,
            contents_url: String,
            contributors_url: String,
            deployments_url: String,
            downloads_url: String,
            events_url: String,
            forks_url: String,
            git_commits_url: String,
            git_refs_url: String,
            git_tags_url: String,
            git_url: String,
            issue_comment_url: String,
            issue_events_url: String,
            issues_url: String,
            keys_url: String,
            labels_url: String,
            languages_url: String,
            merges_url: String,
            milestones_url: String,
            notifications_url: String,
            pulls_url: String,
            releases_url: String,
            ssh_url: String,
            stargazers_url: String,
            statuses_url: String,
            subscribers_url: String,
            subscription_url: String,
            tags_url: String,
            teams_url: String,
            trees_url: String,
            clone_url: String,
            mirror_url: Option<String>,
            hooks_url: String,
            svn_url: String,
            homepage: Option<String>,
            language: Option<String>,
            forks_count: i64,
            stargazers_count: i64,
            watchers_count: i64,
            size: i64,
            default_branch: String,
            open_issues_count: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_template: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            topics: Option<Vec<String>>,
            has_issues: bool,
            has_projects: bool,
            has_wiki: bool,
            has_pages: bool,
            has_downloads: bool,
            archived: bool,
            disabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            visibility: Option<String>,
            pushed_at: Option<String>,
            created_at: Option<String>,
            updated_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_rebase_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            template_repository: Option<RepositoryTemplateRepository>,
            #[serde(skip_serializing_if = "Option::is_none")]
            temp_clone_token: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_squash_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_auto_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            delete_branch_on_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_merge_commit: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_forking: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subscribers_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            network_count: Option<i64>,
            open_issues: i64,
            watchers: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            master_branch: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            starred_at: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct InstallationToken {
            token: String,
            expires_at: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<crate::components::schemas::AppPermissions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repository_selection: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repositories: Option<Vec<crate::components::schemas::Repository>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            single_file: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_multiple_single_files: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            single_file_paths: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ApplicationGrantApp {
            client_id: String,
            name: String,
            url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ApplicationGrant {
            id: i64,
            url: String,
            app: ApplicationGrantApp,
            created_at: String,
            updated_at: String,
            scopes: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            user: Option<crate::components::schemas::NullableSimpleUser>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableScopedInstallation {
            permissions: crate::components::schemas::AppPermissions,
            repository_selection: String,
            single_file_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_multiple_single_files: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            single_file_paths: Option<Vec<String>>,
            repositories_url: String,
            account: crate::components::schemas::SimpleUser,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct AuthorizationApp {
            client_id: String,
            name: String,
            url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Authorization {
            id: i64,
            url: String,
            scopes: Option<Vec<String>>,
            token: String,
            token_last_eight: Option<String>,
            hashed_token: Option<String>,
            app: AuthorizationApp,
            note: Option<String>,
            note_url: Option<String>,
            updated_at: String,
            created_at: String,
            fingerprint: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            user: Option<crate::components::schemas::NullableSimpleUser>,
            #[serde(skip_serializing_if = "Option::is_none")]
            installation: Option<crate::components::schemas::NullableScopedInstallation>,
            expires_at: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodeOfConduct {
            key: String,
            name: String,
            url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            body: Option<String>,
            html_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ActionsEnterprisePermissions {
            enabled_organizations: crate::components::schemas::EnabledOrganizations,
            #[serde(skip_serializing_if = "Option::is_none")]
            selected_organizations_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allowed_actions: Option<crate::components::schemas::AllowedActions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            selected_actions_url: Option<crate::components::schemas::SelectedActionsUrl>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct OrganizationSimple {
            login: String,
            id: i64,
            node_id: String,
            url: String,
            repos_url: String,
            events_url: String,
            hooks_url: String,
            issues_url: String,
            members_url: String,
            public_members_url: String,
            avatar_url: String,
            description: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct SelectedActions {
            #[serde(skip_serializing_if = "Option::is_none")]
            github_owned_allowed: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            verified_allowed: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            patterns_allowed: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RunnerGroupsEnterprise {
            id: i64,
            name: String,
            visibility: String,
            default: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            selected_organizations_url: Option<String>,
            runners_url: String,
            allows_public_repositories: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RunnerLabels {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            type: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Runner {
            id: i64,
            name: String,
            os: String,
            status: String,
            busy: bool,
            labels: Vec<RunnerLabels>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RunnerApplication {
            os: String,
            architecture: String,
            download_url: String,
            filename: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            temp_download_token: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sha256_checksum: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct AuthenticationTokenPermissions;

        #[derive(Debug, Serialize, Deserialize)]
        struct AuthenticationToken {
            token: String,
            expires_at: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<AuthenticationTokenPermissions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repositories: Option<Vec<crate::components::schemas::Repository>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            single_file: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repository_selection: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct AuditLogEventActorLocation {
            #[serde(skip_serializing_if = "Option::is_none")]
            country_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct AuditLogEventData;

        #[derive(Debug, Serialize, Deserialize)]
        struct AuditLogEventConfig;

        #[derive(Debug, Serialize, Deserialize)]
        struct AuditLogEventConfigWas;

        #[derive(Debug, Serialize, Deserialize)]
        struct AuditLogEventEvents;

        #[derive(Debug, Serialize, Deserialize)]
        struct AuditLogEventEventsWere;

        #[derive(Debug, Serialize, Deserialize)]
        struct AuditLogEvent {
            #[serde(skip_serializing_if = "Option::is_none")]
            @timestamp: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            action: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            active: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            active_was: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            actor: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            actor_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            actor_location: Option<AuditLogEventActorLocation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            data: Option<AuditLogEventData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            org_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            blocked_user: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            business: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            config: Option<Vec<AuditLogEventConfig>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            config_was: Option<Vec<AuditLogEventConfigWas>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content_type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created_at: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            deploy_key_fingerprint: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            _document_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            emoji: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            events: Option<Vec<AuditLogEventEvents>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            events_were: Option<Vec<AuditLogEventEventsWere>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            explanation: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            fingerprint: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            hook_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            limited_availability: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            old_user: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            openssh_public_key: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            org: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            previous_visibility: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            read_only: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repo: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repository: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repository_public: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            target_login: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            team: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            transport_protocol: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            transport_protocol_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            user: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            visibility: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ActionsBillingUsageMinutesUsedBreakdown {
            #[serde(skip_serializing_if = "Option::is_none")]
            UBUNTU: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            MACOS: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            WINDOWS: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ActionsBillingUsage {
            total_minutes_used: i64,
            total_paid_minutes_used: i64,
            included_minutes: i64,
            minutes_used_breakdown: ActionsBillingUsageMinutesUsedBreakdown,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PackagesBillingUsage {
            total_gigabytes_bandwidth_used: i64,
            total_paid_gigabytes_bandwidth_used: i64,
            included_gigabytes_bandwidth: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CombinedBillingUsage {
            days_left_in_billing_cycle: i64,
            estimated_paid_storage_for_month: i64,
            estimated_storage_for_month: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Actor {
            id: i64,
            login: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            display_login: Option<String>,
            gravatar_id: Option<String>,
            url: String,
            avatar_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableMilestone {
            url: String,
            html_url: String,
            labels_url: String,
            id: i64,
            node_id: String,
            number: i64,
            state: String,
            title: String,
            description: Option<String>,
            creator: crate::components::schemas::NullableSimpleUser,
            open_issues: i64,
            closed_issues: i64,
            created_at: String,
            updated_at: String,
            closed_at: Option<String>,
            due_on: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableIntegrationPermissions {
            #[serde(skip_serializing_if = "Option::is_none")]
            issues: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            checks: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            metadata: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            contents: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            deployments: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableIntegration {
            id: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            slug: Option<String>,
            node_id: String,
            owner: crate::components::schemas::NullableSimpleUser,
            name: String,
            description: Option<String>,
            external_url: String,
            html_url: String,
            created_at: String,
            updated_at: String,
            permissions: NullableIntegrationPermissions,
            events: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            installations_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            client_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            client_secret: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook_secret: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pem: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ReactionRollup {
            url: String,
            total_count: i64,
            +1: i64,
            -1: i64,
            laugh: i64,
            confused: i64,
            heart: i64,
            hooray: i64,
            eyes: i64,
            rocket: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct IssueLabels {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            color: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            default: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        enum IssueLabelsOneOf {
            String(String),
            IssueLabels(IssueLabels),
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct IssuePullRequest {
            #[serde(skip_serializing_if = "Option::is_none")]
            merged_at: Option<String>,
            diff_url: Option<String>,
            html_url: Option<String>,
            patch_url: Option<String>,
            url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Issue {
            id: i64,
            node_id: String,
            url: String,
            repository_url: String,
            labels_url: String,
            comments_url: String,
            events_url: String,
            html_url: String,
            number: i64,
            state: String,
            title: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            body: Option<String>,
            user: crate::components::schemas::NullableSimpleUser,
            labels: Vec<IssueLabelsOneOf>,
            assignee: crate::components::schemas::NullableSimpleUser,
            #[serde(skip_serializing_if = "Option::is_none")]
            assignees: Option<Vec<crate::components::schemas::SimpleUser>>,
            milestone: crate::components::schemas::NullableMilestone,
            locked: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            active_lock_reason: Option<String>,
            comments: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            pull_request: Option<IssuePullRequest>,
            closed_at: Option<String>,
            created_at: String,
            updated_at: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            closed_by: Option<crate::components::schemas::NullableSimpleUser>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_html: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            timeline_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repository: Option<crate::components::schemas::Repository>,
            #[serde(skip_serializing_if = "Option::is_none")]
            performed_via_github_app: Option<crate::components::schemas::NullableIntegration>,
            author_association: crate::components::schemas::AuthorAssociation,
            #[serde(skip_serializing_if = "Option::is_none")]
            reactions: Option<crate::components::schemas::ReactionRollup>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct IssueComment {
            id: i64,
            node_id: String,
            url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            body: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_html: Option<String>,
            html_url: String,
            user: crate::components::schemas::NullableSimpleUser,
            created_at: String,
            updated_at: String,
            issue_url: String,
            author_association: crate::components::schemas::AuthorAssociation,
            #[serde(skip_serializing_if = "Option::is_none")]
            performed_via_github_app: Option<crate::components::schemas::NullableIntegration>,
            #[serde(skip_serializing_if = "Option::is_none")]
            reactions: Option<crate::components::schemas::ReactionRollup>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct EventRepo {
            id: i64,
            name: String,
            url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct EventPayloadPages {
            #[serde(skip_serializing_if = "Option::is_none")]
            page_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            summary: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            action: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sha: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct EventPayload {
            #[serde(skip_serializing_if = "Option::is_none")]
            action: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            issue: Option<crate::components::schemas::Issue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            comment: Option<crate::components::schemas::IssueComment>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pages: Option<Vec<EventPayloadPages>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Event {
            id: String,
            type: Option<String>,
            actor: crate::components::schemas::Actor,
            repo: EventRepo,
            #[serde(skip_serializing_if = "Option::is_none")]
            org: Option<crate::components::schemas::Actor>,
            payload: EventPayload,
            public: bool,
            created_at: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct LinkWithType {
            href: String,
            type: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct FeedLinks {
            timeline: crate::components::schemas::LinkWithType,
            user: crate::components::schemas::LinkWithType,
            #[serde(skip_serializing_if = "Option::is_none")]
            security_advisories: Option<crate::components::schemas::LinkWithType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            current_user: Option<crate::components::schemas::LinkWithType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            current_user_public: Option<crate::components::schemas::LinkWithType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            current_user_actor: Option<crate::components::schemas::LinkWithType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            current_user_organization: Option<crate::components::schemas::LinkWithType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            current_user_organizations: Option<Vec<crate::components::schemas::LinkWithType>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Feed {
            timeline_url: String,
            user_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            current_user_public_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            current_user_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            current_user_actor_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            current_user_organization_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            current_user_organization_urls: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            security_advisories_url: Option<String>,
            _links: FeedLinks,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BaseGistFiles;

        #[derive(Debug, Serialize, Deserialize)]
        struct BaseGist {
            url: String,
            forks_url: String,
            commits_url: String,
            id: String,
            node_id: String,
            git_pull_url: String,
            git_push_url: String,
            html_url: String,
            files: BaseGistFiles,
            public: bool,
            created_at: String,
            updated_at: String,
            description: Option<String>,
            comments: i64,
            user: crate::components::schemas::NullableSimpleUser,
            comments_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            owner: Option<crate::components::schemas::SimpleUser>,
            #[serde(skip_serializing_if = "Option::is_none")]
            truncated: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            forks: Option<Vec<>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            history: Option<Vec<>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PublicUserPlan {
            collaborators: i64,
            name: String,
            space: i64,
            private_repos: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PublicUser {
            login: String,
            id: i64,
            node_id: String,
            avatar_url: String,
            gravatar_id: Option<String>,
            url: String,
            html_url: String,
            followers_url: String,
            following_url: String,
            gists_url: String,
            starred_url: String,
            subscriptions_url: String,
            organizations_url: String,
            repos_url: String,
            events_url: String,
            received_events_url: String,
            type: String,
            site_admin: bool,
            name: Option<String>,
            company: Option<String>,
            blog: Option<String>,
            location: Option<String>,
            email: Option<String>,
            hireable: Option<bool>,
            bio: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            twitter_username: Option<String>,
            public_repos: i64,
            public_gists: i64,
            followers: i64,
            following: i64,
            created_at: String,
            updated_at: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            plan: Option<PublicUserPlan>,
            #[serde(skip_serializing_if = "Option::is_none")]
            suspended_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            private_gists: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            total_private_repos: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            owned_private_repos: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            disk_usage: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            collaborators: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GistHistoryChangeStatus {
            #[serde(skip_serializing_if = "Option::is_none")]
            total: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            additions: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            deletions: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GistHistory {
            #[serde(skip_serializing_if = "Option::is_none")]
            user: Option<crate::components::schemas::NullableSimpleUser>,
            #[serde(skip_serializing_if = "Option::is_none")]
            version: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            committed_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            change_status: Option<GistHistoryChangeStatus>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GistSimpleForks {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            user: Option<crate::components::schemas::PublicUser>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            updated_at: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GistSimpleForkOfFiles;

        #[derive(Debug, Serialize, Deserialize)]
        struct GistSimpleForkOf {
            url: String,
            forks_url: String,
            commits_url: String,
            id: String,
            node_id: String,
            git_pull_url: String,
            git_push_url: String,
            html_url: String,
            files: GistSimpleForkOfFiles,
            public: bool,
            created_at: String,
            updated_at: String,
            description: Option<String>,
            comments: i64,
            user: crate::components::schemas::NullableSimpleUser,
            comments_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            owner: Option<crate::components::schemas::NullableSimpleUser>,
            #[serde(skip_serializing_if = "Option::is_none")]
            truncated: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            forks: Option<Vec<>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            history: Option<Vec<>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GistSimpleFiles;

        #[derive(Debug, Serialize, Deserialize)]
        struct GistSimple {
            #[serde(skip_serializing_if = "Option::is_none")]
            forks: Option<Vec<GistSimpleForks>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            history: Option<Vec<crate::components::schemas::GistHistory>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            fork_of: Option<GistSimpleForkOf>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            forks_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            commits_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            git_pull_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            git_push_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            files: Option<GistSimpleFiles>,
            #[serde(skip_serializing_if = "Option::is_none")]
            public: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            updated_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            comments: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            user: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            comments_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            owner: Option<crate::components::schemas::SimpleUser>,
            #[serde(skip_serializing_if = "Option::is_none")]
            truncated: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GistComment {
            id: i64,
            node_id: String,
            url: String,
            body: String,
            user: crate::components::schemas::NullableSimpleUser,
            created_at: String,
            updated_at: String,
            author_association: crate::components::schemas::AuthorAssociation,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GistCommitChangeStatus {
            #[serde(skip_serializing_if = "Option::is_none")]
            total: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            additions: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            deletions: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GistCommit {
            url: String,
            version: String,
            user: crate::components::schemas::NullableSimpleUser,
            change_status: GistCommitChangeStatus,
            committed_at: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GitignoreTemplate {
            name: String,
            source: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct LicenseSimple {
            key: String,
            name: String,
            url: Option<String>,
            spdx_id: Option<String>,
            node_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct License {
            key: String,
            name: String,
            spdx_id: Option<String>,
            url: Option<String>,
            node_id: String,
            html_url: String,
            description: String,
            implementation: String,
            permissions: Vec<String>,
            conditions: Vec<String>,
            limitations: Vec<String>,
            body: String,
            featured: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct MarketplaceListingPlan {
            url: String,
            accounts_url: String,
            id: i64,
            number: i64,
            name: String,
            description: String,
            monthly_price_in_cents: i64,
            yearly_price_in_cents: i64,
            price_model: String,
            has_free_trial: bool,
            unit_name: Option<String>,
            state: String,
            bullets: Vec<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct MarketplacePurchaseMarketplacePendingChange {
            #[serde(skip_serializing_if = "Option::is_none")]
            is_installed: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            effective_date: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            unit_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            plan: Option<crate::components::schemas::MarketplaceListingPlan>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct MarketplacePurchaseMarketplacePurchase {
            #[serde(skip_serializing_if = "Option::is_none")]
            billing_cycle: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            next_billing_date: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_installed: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            unit_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            on_free_trial: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            free_trial_ends_on: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            updated_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            plan: Option<crate::components::schemas::MarketplaceListingPlan>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct MarketplacePurchase {
            url: String,
            type: String,
            id: i64,
            login: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            organization_billing_email: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            email: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            marketplace_pending_change: Option<MarketplacePurchaseMarketplacePendingChange>,
            marketplace_purchase: MarketplacePurchaseMarketplacePurchase,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ApiOverviewSshKeyFingerprints {
            #[serde(skip_serializing_if = "Option::is_none")]
            SHA256_RSA: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            SHA256_DSA: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            SHA256_ECDSA: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            SHA256_ED25519: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ApiOverview {
            verifiable_password_authentication: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            ssh_key_fingerprints: Option<ApiOverviewSshKeyFingerprints>,
            #[serde(skip_serializing_if = "Option::is_none")]
            hooks: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            web: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            api: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            git: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            packages: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pages: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            importer: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            actions: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            dependabot: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableRepositoryPermissions {
            admin: bool,
            pull: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            triage: Option<bool>,
            push: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            maintain: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableRepositoryTemplateRepositoryOwner {
            #[serde(skip_serializing_if = "Option::is_none")]
            login: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            avatar_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            gravatar_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            followers_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            following_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            gists_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            starred_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subscriptions_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            organizations_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repos_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            events_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            received_events_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            site_admin: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableRepositoryTemplateRepositoryPermissions {
            #[serde(skip_serializing_if = "Option::is_none")]
            admin: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            maintain: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            push: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            triage: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pull: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableRepositoryTemplateRepository {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            full_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            owner: Option<NullableRepositoryTemplateRepositoryOwner>,
            #[serde(skip_serializing_if = "Option::is_none")]
            private: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            fork: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            archive_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            assignees_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            blobs_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            branches_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            collaborators_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            comments_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            commits_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            compare_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            contents_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            contributors_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            deployments_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            downloads_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            events_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            forks_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            git_commits_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            git_refs_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            git_tags_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            git_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            issue_comment_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            issue_events_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            issues_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            keys_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            labels_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            languages_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            merges_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            milestones_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            notifications_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pulls_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            releases_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ssh_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            stargazers_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            statuses_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subscribers_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subscription_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            tags_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            teams_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            trees_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            clone_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            mirror_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            hooks_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            svn_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            homepage: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            language: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            forks_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            stargazers_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            watchers_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            size: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            default_branch: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            open_issues_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_template: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            topics: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_issues: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_projects: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_wiki: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_pages: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_downloads: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            archived: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            disabled: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            visibility: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pushed_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            updated_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<NullableRepositoryTemplateRepositoryPermissions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_rebase_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            temp_clone_token: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_squash_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_auto_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            delete_branch_on_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_merge_commit: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subscribers_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            network_count: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableRepository {
            id: i64,
            node_id: String,
            name: String,
            full_name: String,
            license: crate::components::schemas::NullableLicenseSimple,
            #[serde(skip_serializing_if = "Option::is_none")]
            organization: Option<crate::components::schemas::NullableSimpleUser>,
            forks: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<NullableRepositoryPermissions>,
            owner: crate::components::schemas::SimpleUser,
            private: bool,
            html_url: String,
            description: Option<String>,
            fork: bool,
            url: String,
            archive_url: String,
            assignees_url: String,
            blobs_url: String,
            branches_url: String,
            collaborators_url: String,
            comments_url: String,
            commits_url: String,
            compare_url: String,
            contents_url: String,
            contributors_url: String,
            deployments_url: String,
            downloads_url: String,
            events_url: String,
            forks_url: String,
            git_commits_url: String,
            git_refs_url: String,
            git_tags_url: String,
            git_url: String,
            issue_comment_url: String,
            issue_events_url: String,
            issues_url: String,
            keys_url: String,
            labels_url: String,
            languages_url: String,
            merges_url: String,
            milestones_url: String,
            notifications_url: String,
            pulls_url: String,
            releases_url: String,
            ssh_url: String,
            stargazers_url: String,
            statuses_url: String,
            subscribers_url: String,
            subscription_url: String,
            tags_url: String,
            teams_url: String,
            trees_url: String,
            clone_url: String,
            mirror_url: Option<String>,
            hooks_url: String,
            svn_url: String,
            homepage: Option<String>,
            language: Option<String>,
            forks_count: i64,
            stargazers_count: i64,
            watchers_count: i64,
            size: i64,
            default_branch: String,
            open_issues_count: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_template: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            topics: Option<Vec<String>>,
            has_issues: bool,
            has_projects: bool,
            has_wiki: bool,
            has_pages: bool,
            has_downloads: bool,
            archived: bool,
            disabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            visibility: Option<String>,
            pushed_at: Option<String>,
            created_at: Option<String>,
            updated_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_rebase_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            template_repository: Option<NullableRepositoryTemplateRepository>,
            #[serde(skip_serializing_if = "Option::is_none")]
            temp_clone_token: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_squash_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_auto_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            delete_branch_on_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_merge_commit: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_forking: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subscribers_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            network_count: Option<i64>,
            open_issues: i64,
            watchers: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            master_branch: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            starred_at: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct MinimalRepositoryPermissions {
            #[serde(skip_serializing_if = "Option::is_none")]
            admin: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            maintain: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            push: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            triage: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pull: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct MinimalRepositoryLicense {
            #[serde(skip_serializing_if = "Option::is_none")]
            key: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            spdx_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct MinimalRepository {
            id: i64,
            node_id: String,
            name: String,
            full_name: String,
            owner: crate::components::schemas::SimpleUser,
            private: bool,
            html_url: String,
            description: Option<String>,
            fork: bool,
            url: String,
            archive_url: String,
            assignees_url: String,
            blobs_url: String,
            branches_url: String,
            collaborators_url: String,
            comments_url: String,
            commits_url: String,
            compare_url: String,
            contents_url: String,
            contributors_url: String,
            deployments_url: String,
            downloads_url: String,
            events_url: String,
            forks_url: String,
            git_commits_url: String,
            git_refs_url: String,
            git_tags_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            git_url: Option<String>,
            issue_comment_url: String,
            issue_events_url: String,
            issues_url: String,
            keys_url: String,
            labels_url: String,
            languages_url: String,
            merges_url: String,
            milestones_url: String,
            notifications_url: String,
            pulls_url: String,
            releases_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            ssh_url: Option<String>,
            stargazers_url: String,
            statuses_url: String,
            subscribers_url: String,
            subscription_url: String,
            tags_url: String,
            teams_url: String,
            trees_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            clone_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            mirror_url: Option<String>,
            hooks_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            svn_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            homepage: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            language: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            forks_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            stargazers_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            watchers_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            size: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            default_branch: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            open_issues_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_template: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            topics: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_issues: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_projects: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_wiki: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_pages: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_downloads: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            archived: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            disabled: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            visibility: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pushed_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            updated_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<MinimalRepositoryPermissions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            role_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            template_repository: Option<crate::components::schemas::NullableRepository>,
            #[serde(skip_serializing_if = "Option::is_none")]
            temp_clone_token: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            delete_branch_on_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subscribers_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            network_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            code_of_conduct: Option<crate::components::schemas::CodeOfConduct>,
            #[serde(skip_serializing_if = "Option::is_none")]
            license: Option<MinimalRepositoryLicense>,
            #[serde(skip_serializing_if = "Option::is_none")]
            forks: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            open_issues: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            watchers: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_forking: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ThreadSubject {
            title: String,
            url: String,
            latest_comment_url: String,
            type: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Thread {
            id: String,
            repository: crate::components::schemas::MinimalRepository,
            subject: ThreadSubject,
            reason: String,
            unread: bool,
            updated_at: String,
            last_read_at: Option<String>,
            url: String,
            subscription_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ThreadSubscription {
            subscribed: bool,
            ignored: bool,
            reason: Option<String>,
            created_at: Option<String>,
            url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            thread_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repository_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct OrganizationCustomRepositoryRole {
            id: i64,
            name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct OrganizationFullPlan {
            name: String,
            space: i64,
            private_repos: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            filled_seats: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            seats: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct OrganizationFull {
            login: String,
            id: i64,
            node_id: String,
            url: String,
            repos_url: String,
            events_url: String,
            hooks_url: String,
            issues_url: String,
            members_url: String,
            public_members_url: String,
            avatar_url: String,
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            company: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            blog: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            location: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            email: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            twitter_username: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_verified: Option<bool>,
            has_organization_projects: bool,
            has_repository_projects: bool,
            public_repos: i64,
            public_gists: i64,
            followers: i64,
            following: i64,
            html_url: String,
            created_at: String,
            type: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            total_private_repos: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            owned_private_repos: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            private_gists: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            disk_usage: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            collaborators: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            billing_email: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            plan: Option<OrganizationFullPlan>,
            #[serde(skip_serializing_if = "Option::is_none")]
            default_repository_permission: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            members_can_create_repositories: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            two_factor_requirement_enabled: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            members_allowed_repository_creation_type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            members_can_create_public_repositories: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            members_can_create_private_repositories: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            members_can_create_internal_repositories: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            members_can_create_pages: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            members_can_create_public_pages: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            members_can_create_private_pages: Option<bool>,
            updated_at: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ActionsOrganizationPermissions {
            enabled_repositories: crate::components::schemas::EnabledRepositories,
            #[serde(skip_serializing_if = "Option::is_none")]
            selected_repositories_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allowed_actions: Option<crate::components::schemas::AllowedActions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            selected_actions_url: Option<crate::components::schemas::SelectedActionsUrl>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RunnerGroupsOrg {
            id: i64,
            name: String,
            visibility: String,
            default: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            selected_repositories_url: Option<String>,
            runners_url: String,
            inherited: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            inherited_allows_public_repositories: Option<bool>,
            allows_public_repositories: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct OrganizationActionsSecret {
            name: String,
            created_at: String,
            updated_at: String,
            visibility: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            selected_repositories_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ActionsPublicKey {
            key_id: String,
            key: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created_at: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct EmptyObject;

        #[derive(Debug, Serialize, Deserialize)]
        struct CredentialAuthorization {
            login: String,
            credential_id: i64,
            credential_type: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            token_last_eight: Option<String>,
            credential_authorized_at: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            scopes: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            fingerprint: Option<String>,
            credential_accessed_at: Option<String>,
            authorized_credential_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            authorized_credential_title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            authorized_credential_note: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            authorized_credential_expires_at: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ExternalGroupTeams {
            team_id: i64,
            team_name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ExternalGroupMembers {
            member_id: i64,
            member_login: String,
            member_name: String,
            member_email: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ExternalGroup {
            group_id: i64,
            group_name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            updated_at: Option<String>,
            teams: Vec<ExternalGroupTeams>,
            members: Vec<ExternalGroupMembers>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ExternalGroupsGroups {
            group_id: i64,
            group_name: String,
            updated_at: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ExternalGroups {
            #[serde(skip_serializing_if = "Option::is_none")]
            groups: Option<Vec<ExternalGroupsGroups>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct OrganizationInvitation {
            id: i64,
            login: Option<String>,
            email: Option<String>,
            role: String,
            created_at: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            failed_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            failed_reason: Option<String>,
            inviter: crate::components::schemas::SimpleUser,
            team_count: i64,
            node_id: String,
            invitation_teams_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct OrgHookConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            insecure_ssl: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content_type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            secret: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct OrgHook {
            id: i64,
            url: String,
            ping_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            deliveries_url: Option<String>,
            name: String,
            events: Vec<String>,
            active: bool,
            config: OrgHookConfig,
            updated_at: String,
            created_at: String,
            type: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct InteractionLimitResponse {
            limit: crate::components::schemas::InteractionGroup,
            origin: String,
            expires_at: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct InteractionLimit {
            limit: crate::components::schemas::InteractionGroup,
            #[serde(skip_serializing_if = "Option::is_none")]
            expiry: Option<crate::components::schemas::InteractionExpiry>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableTeamSimple {
            id: i64,
            node_id: String,
            url: String,
            members_url: String,
            name: String,
            description: Option<String>,
            permission: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            privacy: Option<String>,
            html_url: String,
            repositories_url: String,
            slug: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            ldap_dn: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TeamPermissions {
            pull: bool,
            triage: bool,
            push: bool,
            maintain: bool,
            admin: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Team {
            id: i64,
            node_id: String,
            name: String,
            slug: String,
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            privacy: Option<String>,
            permission: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<TeamPermissions>,
            url: String,
            html_url: String,
            members_url: String,
            repositories_url: String,
            parent: crate::components::schemas::NullableTeamSimple,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct OrgMembershipPermissions {
            can_create_repository: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct OrgMembership {
            url: String,
            state: String,
            role: String,
            organization_url: String,
            organization: crate::components::schemas::OrganizationSimple,
            user: crate::components::schemas::NullableSimpleUser,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<OrgMembershipPermissions>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Migration {
            id: i64,
            owner: crate::components::schemas::NullableSimpleUser,
            guid: String,
            state: String,
            lock_repositories: bool,
            exclude_metadata: bool,
            exclude_git_data: bool,
            exclude_attachments: bool,
            exclude_releases: bool,
            exclude_owner_projects: bool,
            repositories: Vec<crate::components::schemas::Repository>,
            url: String,
            created_at: String,
            updated_at: String,
            node_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            archive_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            exclude: Option<Vec<>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableMinimalRepositoryPermissions {
            #[serde(skip_serializing_if = "Option::is_none")]
            admin: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            maintain: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            push: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            triage: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pull: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableMinimalRepositoryLicense {
            #[serde(skip_serializing_if = "Option::is_none")]
            key: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            spdx_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableMinimalRepository {
            id: i64,
            node_id: String,
            name: String,
            full_name: String,
            owner: crate::components::schemas::SimpleUser,
            private: bool,
            html_url: String,
            description: Option<String>,
            fork: bool,
            url: String,
            archive_url: String,
            assignees_url: String,
            blobs_url: String,
            branches_url: String,
            collaborators_url: String,
            comments_url: String,
            commits_url: String,
            compare_url: String,
            contents_url: String,
            contributors_url: String,
            deployments_url: String,
            downloads_url: String,
            events_url: String,
            forks_url: String,
            git_commits_url: String,
            git_refs_url: String,
            git_tags_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            git_url: Option<String>,
            issue_comment_url: String,
            issue_events_url: String,
            issues_url: String,
            keys_url: String,
            labels_url: String,
            languages_url: String,
            merges_url: String,
            milestones_url: String,
            notifications_url: String,
            pulls_url: String,
            releases_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            ssh_url: Option<String>,
            stargazers_url: String,
            statuses_url: String,
            subscribers_url: String,
            subscription_url: String,
            tags_url: String,
            teams_url: String,
            trees_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            clone_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            mirror_url: Option<String>,
            hooks_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            svn_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            homepage: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            language: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            forks_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            stargazers_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            watchers_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            size: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            default_branch: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            open_issues_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_template: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            topics: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_issues: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_projects: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_wiki: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_pages: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_downloads: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            archived: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            disabled: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            visibility: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pushed_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            updated_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<NullableMinimalRepositoryPermissions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            role_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            template_repository: Option<crate::components::schemas::NullableRepository>,
            #[serde(skip_serializing_if = "Option::is_none")]
            temp_clone_token: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            delete_branch_on_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subscribers_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            network_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            code_of_conduct: Option<crate::components::schemas::CodeOfConduct>,
            #[serde(skip_serializing_if = "Option::is_none")]
            license: Option<NullableMinimalRepositoryLicense>,
            #[serde(skip_serializing_if = "Option::is_none")]
            forks: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            open_issues: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            watchers: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_forking: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Package {
            id: i64,
            name: String,
            package_type: String,
            url: String,
            html_url: String,
            version_count: i64,
            visibility: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            owner: Option<crate::components::schemas::NullableSimpleUser>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repository: Option<crate::components::schemas::NullableMinimalRepository>,
            created_at: String,
            updated_at: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PackageVersionMetadataContainer {
            tags: Vec<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PackageVersionMetadataDocker {
            #[serde(skip_serializing_if = "Option::is_none")]
            tag: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PackageVersionMetadata {
            package_type: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            container: Option<PackageVersionMetadataContainer>,
            #[serde(skip_serializing_if = "Option::is_none")]
            docker: Option<PackageVersionMetadataDocker>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PackageVersion {
            id: i64,
            name: String,
            url: String,
            package_html_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            license: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            created_at: String,
            updated_at: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            deleted_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            metadata: Option<PackageVersionMetadata>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Project {
            owner_url: String,
            url: String,
            html_url: String,
            columns_url: String,
            id: i64,
            node_id: String,
            name: String,
            body: Option<String>,
            number: i64,
            state: String,
            creator: crate::components::schemas::NullableSimpleUser,
            created_at: String,
            updated_at: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            organization_permission: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            private: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct OrganizationSecretScanningAlert {
            #[serde(skip_serializing_if = "Option::is_none")]
            number: Option<crate::components::schemas::AlertNumber>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created_at: Option<crate::components::schemas::AlertCreatedAt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<crate::components::schemas::AlertUrl>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<crate::components::schemas::AlertHtmlUrl>,
            #[serde(skip_serializing_if = "Option::is_none")]
            locations_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            state: Option<crate::components::schemas::SecretScanningAlertState>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolution: Option<crate::components::schemas::SecretScanningAlertResolution>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolved_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolved_by: Option<crate::components::schemas::NullableSimpleUser>,
            #[serde(skip_serializing_if = "Option::is_none")]
            secret_type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            secret: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repository: Option<crate::components::schemas::MinimalRepository>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GroupMappingGroups {
            group_id: String,
            group_name: String,
            group_description: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            status: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            synced_at: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GroupMapping {
            #[serde(skip_serializing_if = "Option::is_none")]
            groups: Option<Vec<GroupMappingGroups>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TeamFull {
            id: i64,
            node_id: String,
            url: String,
            html_url: String,
            name: String,
            slug: String,
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            privacy: Option<String>,
            permission: String,
            members_url: String,
            repositories_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            parent: Option<crate::components::schemas::NullableTeamSimple>,
            members_count: i64,
            repos_count: i64,
            created_at: String,
            updated_at: String,
            organization: crate::components::schemas::OrganizationFull,
            #[serde(skip_serializing_if = "Option::is_none")]
            ldap_dn: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TeamDiscussion {
            author: crate::components::schemas::NullableSimpleUser,
            body: String,
            body_html: String,
            body_version: String,
            comments_count: i64,
            comments_url: String,
            created_at: String,
            last_edited_at: Option<String>,
            html_url: String,
            node_id: String,
            number: i64,
            pinned: bool,
            private: bool,
            team_url: String,
            title: String,
            updated_at: String,
            url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            reactions: Option<crate::components::schemas::ReactionRollup>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TeamDiscussionComment {
            author: crate::components::schemas::NullableSimpleUser,
            body: String,
            body_html: String,
            body_version: String,
            created_at: String,
            last_edited_at: Option<String>,
            discussion_url: String,
            html_url: String,
            node_id: String,
            number: i64,
            updated_at: String,
            url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            reactions: Option<crate::components::schemas::ReactionRollup>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Reaction {
            id: i64,
            node_id: String,
            user: crate::components::schemas::NullableSimpleUser,
            content: String,
            created_at: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TeamMembership {
            url: String,
            role: String,
            state: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TeamProjectPermissions {
            read: bool,
            write: bool,
            admin: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TeamProject {
            owner_url: String,
            url: String,
            html_url: String,
            columns_url: String,
            id: i64,
            node_id: String,
            name: String,
            body: Option<String>,
            number: i64,
            state: String,
            creator: crate::components::schemas::SimpleUser,
            created_at: String,
            updated_at: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            organization_permission: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            private: Option<bool>,
            permissions: TeamProjectPermissions,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TeamRepositoryPermissions {
            admin: bool,
            pull: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            triage: Option<bool>,
            push: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            maintain: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TeamRepository {
            id: i64,
            node_id: String,
            name: String,
            full_name: String,
            license: crate::components::schemas::NullableLicenseSimple,
            forks: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<TeamRepositoryPermissions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            role_name: Option<String>,
            owner: crate::components::schemas::NullableSimpleUser,
            private: bool,
            html_url: String,
            description: Option<String>,
            fork: bool,
            url: String,
            archive_url: String,
            assignees_url: String,
            blobs_url: String,
            branches_url: String,
            collaborators_url: String,
            comments_url: String,
            commits_url: String,
            compare_url: String,
            contents_url: String,
            contributors_url: String,
            deployments_url: String,
            downloads_url: String,
            events_url: String,
            forks_url: String,
            git_commits_url: String,
            git_refs_url: String,
            git_tags_url: String,
            git_url: String,
            issue_comment_url: String,
            issue_events_url: String,
            issues_url: String,
            keys_url: String,
            labels_url: String,
            languages_url: String,
            merges_url: String,
            milestones_url: String,
            notifications_url: String,
            pulls_url: String,
            releases_url: String,
            ssh_url: String,
            stargazers_url: String,
            statuses_url: String,
            subscribers_url: String,
            subscription_url: String,
            tags_url: String,
            teams_url: String,
            trees_url: String,
            clone_url: String,
            mirror_url: Option<String>,
            hooks_url: String,
            svn_url: String,
            homepage: Option<String>,
            language: Option<String>,
            forks_count: i64,
            stargazers_count: i64,
            watchers_count: i64,
            size: i64,
            default_branch: String,
            open_issues_count: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_template: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            topics: Option<Vec<String>>,
            has_issues: bool,
            has_projects: bool,
            has_wiki: bool,
            has_pages: bool,
            has_downloads: bool,
            archived: bool,
            disabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            visibility: Option<String>,
            pushed_at: Option<String>,
            created_at: Option<String>,
            updated_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_rebase_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            template_repository: Option<crate::components::schemas::NullableRepository>,
            #[serde(skip_serializing_if = "Option::is_none")]
            temp_clone_token: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_squash_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_auto_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            delete_branch_on_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_merge_commit: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_forking: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subscribers_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            network_count: Option<i64>,
            open_issues: i64,
            watchers: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            master_branch: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ProjectCard {
            url: String,
            id: i64,
            node_id: String,
            note: Option<String>,
            creator: crate::components::schemas::NullableSimpleUser,
            created_at: String,
            updated_at: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            archived: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            column_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            project_id: Option<String>,
            column_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            content_url: Option<String>,
            project_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ProjectColumn {
            url: String,
            project_url: String,
            cards_url: String,
            id: i64,
            node_id: String,
            name: String,
            created_at: String,
            updated_at: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ProjectCollaboratorPermission {
            permission: String,
            user: crate::components::schemas::NullableSimpleUser,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RateLimit {
            limit: i64,
            remaining: i64,
            reset: i64,
            used: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RateLimitOverviewResources {
            core: crate::components::schemas::RateLimit,
            #[serde(skip_serializing_if = "Option::is_none")]
            graphql: Option<crate::components::schemas::RateLimit>,
            search: crate::components::schemas::RateLimit,
            #[serde(skip_serializing_if = "Option::is_none")]
            source_import: Option<crate::components::schemas::RateLimit>,
            #[serde(skip_serializing_if = "Option::is_none")]
            integration_manifest: Option<crate::components::schemas::RateLimit>,
            #[serde(skip_serializing_if = "Option::is_none")]
            code_scanning_upload: Option<crate::components::schemas::RateLimit>,
            #[serde(skip_serializing_if = "Option::is_none")]
            actions_runner_registration: Option<crate::components::schemas::RateLimit>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RateLimitOverview {
            resources: RateLimitOverviewResources,
            rate: crate::components::schemas::RateLimit,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodeOfConductSimple {
            url: String,
            key: String,
            name: String,
            html_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct FullRepositoryPermissions {
            admin: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            maintain: Option<bool>,
            push: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            triage: Option<bool>,
            pull: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct FullRepositorySecurityAndAnalysisAdvancedSecurity {
            #[serde(skip_serializing_if = "Option::is_none")]
            status: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct FullRepositorySecurityAndAnalysisSecretScanning {
            #[serde(skip_serializing_if = "Option::is_none")]
            status: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct FullRepositorySecurityAndAnalysis {
            #[serde(skip_serializing_if = "Option::is_none")]
            advanced_security: Option<FullRepositorySecurityAndAnalysisAdvancedSecurity>,
            #[serde(skip_serializing_if = "Option::is_none")]
            secret_scanning: Option<FullRepositorySecurityAndAnalysisSecretScanning>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct FullRepository {
            id: i64,
            node_id: String,
            name: String,
            full_name: String,
            owner: crate::components::schemas::SimpleUser,
            private: bool,
            html_url: String,
            description: Option<String>,
            fork: bool,
            url: String,
            archive_url: String,
            assignees_url: String,
            blobs_url: String,
            branches_url: String,
            collaborators_url: String,
            comments_url: String,
            commits_url: String,
            compare_url: String,
            contents_url: String,
            contributors_url: String,
            deployments_url: String,
            downloads_url: String,
            events_url: String,
            forks_url: String,
            git_commits_url: String,
            git_refs_url: String,
            git_tags_url: String,
            git_url: String,
            issue_comment_url: String,
            issue_events_url: String,
            issues_url: String,
            keys_url: String,
            labels_url: String,
            languages_url: String,
            merges_url: String,
            milestones_url: String,
            notifications_url: String,
            pulls_url: String,
            releases_url: String,
            ssh_url: String,
            stargazers_url: String,
            statuses_url: String,
            subscribers_url: String,
            subscription_url: String,
            tags_url: String,
            teams_url: String,
            trees_url: String,
            clone_url: String,
            mirror_url: Option<String>,
            hooks_url: String,
            svn_url: String,
            homepage: Option<String>,
            language: Option<String>,
            forks_count: i64,
            stargazers_count: i64,
            watchers_count: i64,
            size: i64,
            default_branch: String,
            open_issues_count: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_template: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            topics: Option<Vec<String>>,
            has_issues: bool,
            has_projects: bool,
            has_wiki: bool,
            has_pages: bool,
            has_downloads: bool,
            archived: bool,
            disabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            visibility: Option<String>,
            pushed_at: String,
            created_at: String,
            updated_at: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<FullRepositoryPermissions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_rebase_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            template_repository: Option<crate::components::schemas::NullableRepository>,
            #[serde(skip_serializing_if = "Option::is_none")]
            temp_clone_token: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_squash_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_auto_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            delete_branch_on_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_merge_commit: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_forking: Option<bool>,
            subscribers_count: i64,
            network_count: i64,
            license: crate::components::schemas::NullableLicenseSimple,
            #[serde(skip_serializing_if = "Option::is_none")]
            organization: Option<crate::components::schemas::NullableSimpleUser>,
            #[serde(skip_serializing_if = "Option::is_none")]
            parent: Option<crate::components::schemas::Repository>,
            #[serde(skip_serializing_if = "Option::is_none")]
            source: Option<crate::components::schemas::Repository>,
            forks: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            master_branch: Option<String>,
            open_issues: i64,
            watchers: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            anonymous_access_enabled: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            code_of_conduct: Option<crate::components::schemas::CodeOfConductSimple>,
            #[serde(skip_serializing_if = "Option::is_none")]
            security_and_analysis: Option<FullRepositorySecurityAndAnalysis>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Artifact {
            id: i64,
            node_id: String,
            name: String,
            size_in_bytes: i64,
            url: String,
            archive_download_url: String,
            expired: bool,
            created_at: Option<String>,
            expires_at: Option<String>,
            updated_at: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct JobSteps {
            status: String,
            conclusion: Option<String>,
            name: String,
            number: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            started_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            completed_at: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Job {
            id: i64,
            run_id: i64,
            run_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            run_attempt: Option<i64>,
            node_id: String,
            head_sha: String,
            url: String,
            html_url: Option<String>,
            status: String,
            conclusion: Option<String>,
            started_at: String,
            completed_at: Option<String>,
            name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            steps: Option<Vec<JobSteps>>,
            check_run_url: String,
            labels: Vec<String>,
            runner_id: Option<i64>,
            runner_name: Option<String>,
            runner_group_id: Option<i64>,
            runner_group_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ActionsRepositoryPermissions {
            enabled: crate::components::schemas::ActionsEnabled,
            #[serde(skip_serializing_if = "Option::is_none")]
            allowed_actions: Option<crate::components::schemas::AllowedActions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            selected_actions_url: Option<crate::components::schemas::SelectedActionsUrl>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestMinimalHeadRepo {
            id: i64,
            url: String,
            name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestMinimalHead {
            ref: String,
            sha: String,
            repo: PullRequestMinimalHeadRepo,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestMinimalBaseRepo {
            id: i64,
            url: String,
            name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestMinimalBase {
            ref: String,
            sha: String,
            repo: PullRequestMinimalBaseRepo,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestMinimal {
            id: i64,
            number: i64,
            url: String,
            head: PullRequestMinimalHead,
            base: PullRequestMinimalBase,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableSimpleCommitAuthor {
            name: String,
            email: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableSimpleCommitCommitter {
            name: String,
            email: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableSimpleCommit {
            id: String,
            tree_id: String,
            message: String,
            timestamp: String,
            author: Option<NullableSimpleCommitAuthor>,
            committer: Option<NullableSimpleCommitCommitter>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct WorkflowRun {
            id: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            node_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            check_suite_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            check_suite_node_id: Option<String>,
            head_branch: Option<String>,
            head_sha: String,
            run_number: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            run_attempt: Option<i64>,
            event: String,
            status: Option<String>,
            conclusion: Option<String>,
            workflow_id: i64,
            url: String,
            html_url: String,
            pull_requests: Option<Vec<crate::components::schemas::PullRequestMinimal>>,
            created_at: String,
            updated_at: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            run_started_at: Option<String>,
            jobs_url: String,
            logs_url: String,
            check_suite_url: String,
            artifacts_url: String,
            cancel_url: String,
            rerun_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            previous_attempt_url: Option<String>,
            workflow_url: String,
            head_commit: crate::components::schemas::NullableSimpleCommit,
            repository: crate::components::schemas::MinimalRepository,
            head_repository: crate::components::schemas::MinimalRepository,
            #[serde(skip_serializing_if = "Option::is_none")]
            head_repository_id: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct EnvironmentApprovalsEnvironments {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            updated_at: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct EnvironmentApprovals {
            environments: Vec<EnvironmentApprovalsEnvironments>,
            state: String,
            user: crate::components::schemas::SimpleUser,
            comment: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PendingDeploymentEnvironment {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PendingDeploymentReviewers {
            #[serde(skip_serializing_if = "Option::is_none")]
            type: Option<crate::components::schemas::DeploymentReviewerType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            reviewer: Option<>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PendingDeployment {
            environment: PendingDeploymentEnvironment,
            wait_timer: i64,
            wait_timer_started_at: Option<String>,
            current_user_can_approve: bool,
            reviewers: Vec<PendingDeploymentReviewers>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct DeploymentPayload;

        #[derive(Debug, Serialize, Deserialize)]
        enum DeploymentPayloadOneOf {
            DeploymentPayload(DeploymentPayload),
            String(String),
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Deployment {
            url: String,
            id: i64,
            node_id: String,
            sha: String,
            ref: String,
            task: String,
            payload: DeploymentPayloadOneOf,
            #[serde(skip_serializing_if = "Option::is_none")]
            original_environment: Option<String>,
            environment: String,
            description: Option<String>,
            creator: crate::components::schemas::NullableSimpleUser,
            created_at: String,
            updated_at: String,
            statuses_url: String,
            repository_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            transient_environment: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            production_environment: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            performed_via_github_app: Option<crate::components::schemas::NullableIntegration>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct WorkflowRunUsageBillableUbuntuJobRuns {
            job_id: i64,
            duration_ms: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct WorkflowRunUsageBillableUbuntu {
            total_ms: i64,
            jobs: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            job_runs: Option<Vec<WorkflowRunUsageBillableUbuntuJobRuns>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct WorkflowRunUsageBillableMacosJobRuns {
            job_id: i64,
            duration_ms: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct WorkflowRunUsageBillableMacos {
            total_ms: i64,
            jobs: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            job_runs: Option<Vec<WorkflowRunUsageBillableMacosJobRuns>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct WorkflowRunUsageBillableWindowsJobRuns {
            job_id: i64,
            duration_ms: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct WorkflowRunUsageBillableWindows {
            total_ms: i64,
            jobs: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            job_runs: Option<Vec<WorkflowRunUsageBillableWindowsJobRuns>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct WorkflowRunUsageBillable {
            #[serde(skip_serializing_if = "Option::is_none")]
            UBUNTU: Option<WorkflowRunUsageBillableUbuntu>,
            #[serde(skip_serializing_if = "Option::is_none")]
            MACOS: Option<WorkflowRunUsageBillableMacos>,
            #[serde(skip_serializing_if = "Option::is_none")]
            WINDOWS: Option<WorkflowRunUsageBillableWindows>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct WorkflowRunUsage {
            billable: WorkflowRunUsageBillable,
            #[serde(skip_serializing_if = "Option::is_none")]
            run_duration_ms: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ActionsSecret {
            name: String,
            created_at: String,
            updated_at: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Workflow {
            id: i64,
            node_id: String,
            name: String,
            path: String,
            state: String,
            created_at: String,
            updated_at: String,
            url: String,
            html_url: String,
            badge_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            deleted_at: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct WorkflowUsageBillableUbuntu {
            #[serde(skip_serializing_if = "Option::is_none")]
            total_ms: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct WorkflowUsageBillableMacos {
            #[serde(skip_serializing_if = "Option::is_none")]
            total_ms: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct WorkflowUsageBillableWindows {
            #[serde(skip_serializing_if = "Option::is_none")]
            total_ms: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct WorkflowUsageBillable {
            #[serde(skip_serializing_if = "Option::is_none")]
            UBUNTU: Option<WorkflowUsageBillableUbuntu>,
            #[serde(skip_serializing_if = "Option::is_none")]
            MACOS: Option<WorkflowUsageBillableMacos>,
            #[serde(skip_serializing_if = "Option::is_none")]
            WINDOWS: Option<WorkflowUsageBillableWindows>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct WorkflowUsage {
            billable: WorkflowUsageBillable,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Autolink {
            id: i64,
            key_prefix: String,
            url_template: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ProtectedBranchAdminEnforced {
            url: String,
            enabled: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ProtectedBranchPullRequestReviewDismissalRestrictions {
            #[serde(skip_serializing_if = "Option::is_none")]
            users: Option<Vec<crate::components::schemas::SimpleUser>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            teams: Option<Vec<crate::components::schemas::Team>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            users_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            teams_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ProtectedBranchPullRequestReview {
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            dismissal_restrictions: Option<ProtectedBranchPullRequestReviewDismissalRestrictions>,
            dismiss_stale_reviews: bool,
            require_code_owner_reviews: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            required_approving_review_count: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BranchRestrictionPolicyUsers {
            #[serde(skip_serializing_if = "Option::is_none")]
            login: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            avatar_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            gravatar_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            followers_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            following_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            gists_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            starred_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subscriptions_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            organizations_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repos_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            events_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            received_events_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            site_admin: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BranchRestrictionPolicyTeams {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            slug: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            privacy: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            permission: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            members_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repositories_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            parent: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BranchRestrictionPolicyAppsOwner {
            #[serde(skip_serializing_if = "Option::is_none")]
            login: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repos_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            events_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            hooks_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            issues_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            members_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            public_members_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            avatar_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            gravatar_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            followers_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            following_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            gists_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            starred_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subscriptions_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            organizations_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            received_events_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            site_admin: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BranchRestrictionPolicyAppsPermissions {
            #[serde(skip_serializing_if = "Option::is_none")]
            metadata: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            contents: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            issues: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            single_file: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BranchRestrictionPolicyApps {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            slug: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            owner: Option<BranchRestrictionPolicyAppsOwner>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            external_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            updated_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<BranchRestrictionPolicyAppsPermissions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            events: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BranchRestrictionPolicy {
            url: String,
            users_url: String,
            teams_url: String,
            apps_url: String,
            users: Vec<BranchRestrictionPolicyUsers>,
            teams: Vec<BranchRestrictionPolicyTeams>,
            apps: Vec<BranchRestrictionPolicyApps>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BranchProtectionRequiredStatusChecks {
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            enforcement_level: Option<String>,
            contexts: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            contexts_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            strict: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BranchProtectionRequiredLinearHistory {
            #[serde(skip_serializing_if = "Option::is_none")]
            enabled: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BranchProtectionAllowForcePushes {
            #[serde(skip_serializing_if = "Option::is_none")]
            enabled: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BranchProtectionAllowDeletions {
            #[serde(skip_serializing_if = "Option::is_none")]
            enabled: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BranchProtectionRequiredConversationResolution {
            #[serde(skip_serializing_if = "Option::is_none")]
            enabled: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BranchProtectionRequiredSignatures {
            url: String,
            enabled: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BranchProtection {
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            enabled: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            required_status_checks: Option<BranchProtectionRequiredStatusChecks>,
            #[serde(skip_serializing_if = "Option::is_none")]
            enforce_admins: Option<crate::components::schemas::ProtectedBranchAdminEnforced>,
            #[serde(skip_serializing_if = "Option::is_none")]
            required_pull_request_reviews: Option<crate::components::schemas::ProtectedBranchPullRequestReview>,
            #[serde(skip_serializing_if = "Option::is_none")]
            restrictions: Option<crate::components::schemas::BranchRestrictionPolicy>,
            #[serde(skip_serializing_if = "Option::is_none")]
            required_linear_history: Option<BranchProtectionRequiredLinearHistory>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_force_pushes: Option<BranchProtectionAllowForcePushes>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_deletions: Option<BranchProtectionAllowDeletions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            required_conversation_resolution: Option<BranchProtectionRequiredConversationResolution>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            protection_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            required_signatures: Option<BranchProtectionRequiredSignatures>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ShortBranchCommit {
            sha: String,
            url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ShortBranch {
            name: String,
            commit: ShortBranchCommit,
            protected: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            protection: Option<crate::components::schemas::BranchProtection>,
            #[serde(skip_serializing_if = "Option::is_none")]
            protection_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableGitUser {
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            email: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            date: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Verification {
            verified: bool,
            reason: String,
            payload: Option<String>,
            signature: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct DiffEntry {
            sha: String,
            filename: String,
            status: String,
            additions: i64,
            deletions: i64,
            changes: i64,
            blob_url: String,
            raw_url: String,
            contents_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            patch: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            previous_filename: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CommitCommitTree {
            sha: String,
            url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CommitCommit {
            url: String,
            author: crate::components::schemas::NullableGitUser,
            committer: crate::components::schemas::NullableGitUser,
            message: String,
            comment_count: i64,
            tree: CommitCommitTree,
            #[serde(skip_serializing_if = "Option::is_none")]
            verification: Option<crate::components::schemas::Verification>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CommitParents {
            sha: String,
            url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CommitStats {
            #[serde(skip_serializing_if = "Option::is_none")]
            additions: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            deletions: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            total: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Commit {
            url: String,
            sha: String,
            node_id: String,
            html_url: String,
            comments_url: String,
            commit: CommitCommit,
            author: crate::components::schemas::NullableSimpleUser,
            committer: crate::components::schemas::NullableSimpleUser,
            parents: Vec<CommitParents>,
            #[serde(skip_serializing_if = "Option::is_none")]
            stats: Option<CommitStats>,
            #[serde(skip_serializing_if = "Option::is_none")]
            files: Option<Vec<crate::components::schemas::DiffEntry>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BranchWithProtectionLinks {
            html: String,
            self: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BranchWithProtection {
            name: String,
            commit: crate::components::schemas::Commit,
            _links: BranchWithProtectionLinks,
            protected: bool,
            protection: crate::components::schemas::BranchProtection,
            protection_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pattern: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            required_approving_review_count: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct StatusCheckPolicy {
            url: String,
            strict: bool,
            contexts: Vec<String>,
            contexts_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ProtectedBranchRequiredPullRequestReviewsDismissalRestrictions {
            url: String,
            users_url: String,
            teams_url: String,
            users: Vec<crate::components::schemas::SimpleUser>,
            teams: Vec<crate::components::schemas::Team>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ProtectedBranchRequiredPullRequestReviews {
            url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            dismiss_stale_reviews: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            require_code_owner_reviews: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            required_approving_review_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            dismissal_restrictions: Option<ProtectedBranchRequiredPullRequestReviewsDismissalRestrictions>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ProtectedBranchRequiredSignatures {
            url: String,
            enabled: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ProtectedBranchEnforceAdmins {
            url: String,
            enabled: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ProtectedBranchRequiredLinearHistory {
            enabled: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ProtectedBranchAllowForcePushes {
            enabled: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ProtectedBranchAllowDeletions {
            enabled: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ProtectedBranchRequiredConversationResolution {
            #[serde(skip_serializing_if = "Option::is_none")]
            enabled: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ProtectedBranch {
            url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            required_status_checks: Option<crate::components::schemas::StatusCheckPolicy>,
            #[serde(skip_serializing_if = "Option::is_none")]
            required_pull_request_reviews: Option<ProtectedBranchRequiredPullRequestReviews>,
            #[serde(skip_serializing_if = "Option::is_none")]
            required_signatures: Option<ProtectedBranchRequiredSignatures>,
            #[serde(skip_serializing_if = "Option::is_none")]
            enforce_admins: Option<ProtectedBranchEnforceAdmins>,
            #[serde(skip_serializing_if = "Option::is_none")]
            required_linear_history: Option<ProtectedBranchRequiredLinearHistory>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_force_pushes: Option<ProtectedBranchAllowForcePushes>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_deletions: Option<ProtectedBranchAllowDeletions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            restrictions: Option<crate::components::schemas::BranchRestrictionPolicy>,
            #[serde(skip_serializing_if = "Option::is_none")]
            required_conversation_resolution: Option<ProtectedBranchRequiredConversationResolution>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct DeploymentSimple {
            url: String,
            id: i64,
            node_id: String,
            task: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            original_environment: Option<String>,
            environment: String,
            description: Option<String>,
            created_at: String,
            updated_at: String,
            statuses_url: String,
            repository_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            transient_environment: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            production_environment: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            performed_via_github_app: Option<crate::components::schemas::NullableIntegration>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CheckRunOutput {
            title: Option<String>,
            summary: Option<String>,
            text: Option<String>,
            annotations_count: i64,
            annotations_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CheckRunCheckSuite {
            id: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CheckRun {
            id: i64,
            head_sha: String,
            node_id: String,
            external_id: Option<String>,
            url: String,
            html_url: Option<String>,
            details_url: Option<String>,
            status: String,
            conclusion: Option<String>,
            started_at: Option<String>,
            completed_at: Option<String>,
            output: CheckRunOutput,
            name: String,
            check_suite: Option<CheckRunCheckSuite>,
            app: crate::components::schemas::NullableIntegration,
            pull_requests: Vec<crate::components::schemas::PullRequestMinimal>,
            #[serde(skip_serializing_if = "Option::is_none")]
            deployment: Option<crate::components::schemas::DeploymentSimple>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CheckAnnotation {
            path: String,
            start_line: i64,
            end_line: i64,
            start_column: Option<i64>,
            end_column: Option<i64>,
            annotation_level: Option<String>,
            title: Option<String>,
            message: Option<String>,
            raw_details: Option<String>,
            blob_href: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct SimpleCommitAuthor {
            name: String,
            email: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct SimpleCommitCommitter {
            name: String,
            email: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct SimpleCommit {
            id: String,
            tree_id: String,
            message: String,
            timestamp: String,
            author: Option<SimpleCommitAuthor>,
            committer: Option<SimpleCommitCommitter>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CheckSuite {
            id: i64,
            node_id: String,
            head_branch: Option<String>,
            head_sha: String,
            status: Option<String>,
            conclusion: Option<String>,
            url: Option<String>,
            before: Option<String>,
            after: Option<String>,
            pull_requests: Option<Vec<crate::components::schemas::PullRequestMinimal>>,
            app: crate::components::schemas::NullableIntegration,
            repository: crate::components::schemas::MinimalRepository,
            created_at: Option<String>,
            updated_at: Option<String>,
            head_commit: crate::components::schemas::SimpleCommit,
            latest_check_runs_count: i64,
            check_runs_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CheckSuitePreferencePreferencesAutoTriggerChecks {
            app_id: i64,
            setting: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CheckSuitePreferencePreferences {
            #[serde(skip_serializing_if = "Option::is_none")]
            auto_trigger_checks: Option<Vec<CheckSuitePreferencePreferencesAutoTriggerChecks>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CheckSuitePreference {
            preferences: CheckSuitePreferencePreferences,
            repository: crate::components::schemas::MinimalRepository,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodeScanningAlertRuleSummary {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            severity: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodeScanningAnalysisTool {
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<crate::components::schemas::CodeScanningAnalysisToolName>,
            #[serde(skip_serializing_if = "Option::is_none")]
            version: Option<crate::components::schemas::CodeScanningAnalysisToolVersion>,
            #[serde(skip_serializing_if = "Option::is_none")]
            guid: Option<crate::components::schemas::CodeScanningAnalysisToolGuid>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodeScanningAlertLocation {
            #[serde(skip_serializing_if = "Option::is_none")]
            path: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            start_line: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            end_line: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            start_column: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            end_column: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodeScanningAlertInstanceMessage {
            #[serde(skip_serializing_if = "Option::is_none")]
            text: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodeScanningAlertInstance {
            #[serde(skip_serializing_if = "Option::is_none")]
            ref: Option<crate::components::schemas::CodeScanningRef>,
            #[serde(skip_serializing_if = "Option::is_none")]
            analysis_key: Option<crate::components::schemas::CodeScanningAnalysisAnalysisKey>,
            #[serde(skip_serializing_if = "Option::is_none")]
            environment: Option<crate::components::schemas::CodeScanningAlertEnvironment>,
            #[serde(skip_serializing_if = "Option::is_none")]
            category: Option<crate::components::schemas::CodeScanningAnalysisCategory>,
            #[serde(skip_serializing_if = "Option::is_none")]
            state: Option<crate::components::schemas::CodeScanningAlertState>,
            #[serde(skip_serializing_if = "Option::is_none")]
            commit_sha: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<CodeScanningAlertInstanceMessage>,
            #[serde(skip_serializing_if = "Option::is_none")]
            location: Option<crate::components::schemas::CodeScanningAlertLocation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            classifications: Option<Vec<crate::components::schemas::CodeScanningAlertClassification>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodeScanningAlertItems {
            number: crate::components::schemas::AlertNumber,
            created_at: crate::components::schemas::AlertCreatedAt,
            url: crate::components::schemas::AlertUrl,
            html_url: crate::components::schemas::AlertHtmlUrl,
            instances_url: crate::components::schemas::AlertInstancesUrl,
            state: crate::components::schemas::CodeScanningAlertState,
            dismissed_by: crate::components::schemas::NullableSimpleUser,
            dismissed_at: crate::components::schemas::CodeScanningAlertDismissedAt,
            dismissed_reason: crate::components::schemas::CodeScanningAlertDismissedReason,
            rule: crate::components::schemas::CodeScanningAlertRuleSummary,
            tool: crate::components::schemas::CodeScanningAnalysisTool,
            most_recent_instance: crate::components::schemas::CodeScanningAlertInstance,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodeScanningAlertRule {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            severity: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            security_severity_level: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            full_description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            tags: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            help: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodeScanningAlert {
            number: crate::components::schemas::AlertNumber,
            created_at: crate::components::schemas::AlertCreatedAt,
            url: crate::components::schemas::AlertUrl,
            html_url: crate::components::schemas::AlertHtmlUrl,
            instances_url: crate::components::schemas::AlertInstancesUrl,
            state: crate::components::schemas::CodeScanningAlertState,
            dismissed_by: crate::components::schemas::NullableSimpleUser,
            dismissed_at: crate::components::schemas::CodeScanningAlertDismissedAt,
            dismissed_reason: crate::components::schemas::CodeScanningAlertDismissedReason,
            rule: crate::components::schemas::CodeScanningAlertRule,
            tool: crate::components::schemas::CodeScanningAnalysisTool,
            most_recent_instance: crate::components::schemas::CodeScanningAlertInstance,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodeScanningAnalysis {
            ref: crate::components::schemas::CodeScanningRef,
            commit_sha: crate::components::schemas::CodeScanningAnalysisCommitSha,
            analysis_key: crate::components::schemas::CodeScanningAnalysisAnalysisKey,
            environment: crate::components::schemas::CodeScanningAnalysisEnvironment,
            #[serde(skip_serializing_if = "Option::is_none")]
            category: Option<crate::components::schemas::CodeScanningAnalysisCategory>,
            error: String,
            created_at: crate::components::schemas::CodeScanningAnalysisCreatedAt,
            results_count: i64,
            rules_count: i64,
            id: i64,
            url: crate::components::schemas::CodeScanningAnalysisUrl,
            sarif_id: crate::components::schemas::CodeScanningAnalysisSarifId,
            tool: crate::components::schemas::CodeScanningAnalysisTool,
            deletable: bool,
            warning: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodeScanningAnalysisDeletion {
            next_analysis_url: Option<String>,
            confirm_delete_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodeScanningSarifsReceipt {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<crate::components::schemas::CodeScanningAnalysisSarifId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodeScanningSarifsStatus {
            #[serde(skip_serializing_if = "Option::is_none")]
            processing_status: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            analyses_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableCodespaceMachine {
            name: String,
            display_name: String,
            operating_system: String,
            storage_in_bytes: i64,
            memory_in_bytes: i64,
            cpus: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodespaceGitStatus {
            #[serde(skip_serializing_if = "Option::is_none")]
            ahead: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            behind: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_unpushed_changes: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_uncommitted_changes: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ref: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Codespace {
            id: i64,
            name: String,
            environment_id: Option<String>,
            owner: crate::components::schemas::SimpleUser,
            billable_owner: crate::components::schemas::SimpleUser,
            repository: crate::components::schemas::MinimalRepository,
            machine: crate::components::schemas::NullableCodespaceMachine,
            created_at: String,
            updated_at: String,
            last_used_at: String,
            state: String,
            url: String,
            git_status: CodespaceGitStatus,
            location: String,
            auto_stop_delay_minutes: Option<i64>,
            web_url: String,
            machines_url: String,
            start_url: String,
            stop_url: String,
            pulls_url: Option<String>,
            recent_folders: Vec<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodespaceMachine {
            name: String,
            display_name: String,
            operating_system: String,
            storage_in_bytes: i64,
            memory_in_bytes: i64,
            cpus: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CollaboratorPermissions {
            pull: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            triage: Option<bool>,
            push: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            maintain: Option<bool>,
            admin: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Collaborator {
            login: String,
            id: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            email: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            node_id: String,
            avatar_url: String,
            gravatar_id: Option<String>,
            url: String,
            html_url: String,
            followers_url: String,
            following_url: String,
            gists_url: String,
            starred_url: String,
            subscriptions_url: String,
            organizations_url: String,
            repos_url: String,
            events_url: String,
            received_events_url: String,
            type: String,
            site_admin: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<CollaboratorPermissions>,
            role_name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RepositoryInvitation {
            id: i64,
            repository: crate::components::schemas::MinimalRepository,
            invitee: crate::components::schemas::NullableSimpleUser,
            inviter: crate::components::schemas::NullableSimpleUser,
            permissions: String,
            created_at: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            expired: Option<bool>,
            url: String,
            html_url: String,
            node_id: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableCollaboratorPermissions {
            pull: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            triage: Option<bool>,
            push: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            maintain: Option<bool>,
            admin: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableCollaborator {
            login: String,
            id: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            email: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            node_id: String,
            avatar_url: String,
            gravatar_id: Option<String>,
            url: String,
            html_url: String,
            followers_url: String,
            following_url: String,
            gists_url: String,
            starred_url: String,
            subscriptions_url: String,
            organizations_url: String,
            repos_url: String,
            events_url: String,
            received_events_url: String,
            type: String,
            site_admin: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<NullableCollaboratorPermissions>,
            role_name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RepositoryCollaboratorPermission {
            permission: String,
            role_name: String,
            user: crate::components::schemas::NullableCollaborator,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CommitComment {
            html_url: String,
            url: String,
            id: i64,
            node_id: String,
            body: String,
            path: Option<String>,
            position: Option<i64>,
            line: Option<i64>,
            commit_id: String,
            user: crate::components::schemas::NullableSimpleUser,
            created_at: String,
            updated_at: String,
            author_association: crate::components::schemas::AuthorAssociation,
            #[serde(skip_serializing_if = "Option::is_none")]
            reactions: Option<crate::components::schemas::ReactionRollup>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BranchShortCommit {
            sha: String,
            url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct BranchShort {
            name: String,
            commit: BranchShortCommit,
            protected: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Link {
            href: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct AutoMerge {
            enabled_by: crate::components::schemas::SimpleUser,
            merge_method: String,
            commit_title: String,
            commit_message: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestSimpleLabels {
            id: i64,
            node_id: String,
            url: String,
            name: String,
            description: String,
            color: String,
            default: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestSimpleHead {
            label: String,
            ref: String,
            repo: crate::components::schemas::Repository,
            sha: String,
            user: crate::components::schemas::NullableSimpleUser,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestSimpleBase {
            label: String,
            ref: String,
            repo: crate::components::schemas::Repository,
            sha: String,
            user: crate::components::schemas::NullableSimpleUser,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestSimpleLinks {
            comments: crate::components::schemas::Link,
            commits: crate::components::schemas::Link,
            statuses: crate::components::schemas::Link,
            html: crate::components::schemas::Link,
            issue: crate::components::schemas::Link,
            review_comments: crate::components::schemas::Link,
            review_comment: crate::components::schemas::Link,
            self: crate::components::schemas::Link,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestSimple {
            url: String,
            id: i64,
            node_id: String,
            html_url: String,
            diff_url: String,
            patch_url: String,
            issue_url: String,
            commits_url: String,
            review_comments_url: String,
            review_comment_url: String,
            comments_url: String,
            statuses_url: String,
            number: i64,
            state: String,
            locked: bool,
            title: String,
            user: crate::components::schemas::NullableSimpleUser,
            body: Option<String>,
            labels: Vec<PullRequestSimpleLabels>,
            milestone: crate::components::schemas::NullableMilestone,
            #[serde(skip_serializing_if = "Option::is_none")]
            active_lock_reason: Option<String>,
            created_at: String,
            updated_at: String,
            closed_at: Option<String>,
            merged_at: Option<String>,
            merge_commit_sha: Option<String>,
            assignee: crate::components::schemas::NullableSimpleUser,
            #[serde(skip_serializing_if = "Option::is_none")]
            assignees: Option<Vec<crate::components::schemas::SimpleUser>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            requested_reviewers: Option<Vec<crate::components::schemas::SimpleUser>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            requested_teams: Option<Vec<crate::components::schemas::Team>>,
            head: PullRequestSimpleHead,
            base: PullRequestSimpleBase,
            _links: PullRequestSimpleLinks,
            author_association: crate::components::schemas::AuthorAssociation,
            auto_merge: crate::components::schemas::AutoMerge,
            #[serde(skip_serializing_if = "Option::is_none")]
            draft: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct SimpleCommitStatus {
            description: Option<String>,
            id: i64,
            node_id: String,
            state: String,
            context: String,
            target_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            required: Option<bool>,
            avatar_url: Option<String>,
            url: String,
            created_at: String,
            updated_at: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CombinedCommitStatus {
            state: String,
            statuses: Vec<crate::components::schemas::SimpleCommitStatus>,
            sha: String,
            total_count: i64,
            repository: crate::components::schemas::MinimalRepository,
            commit_url: String,
            url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Status {
            url: String,
            avatar_url: Option<String>,
            id: i64,
            node_id: String,
            state: String,
            description: String,
            target_url: String,
            context: String,
            created_at: String,
            updated_at: String,
            creator: crate::components::schemas::NullableSimpleUser,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableCodeOfConductSimple {
            url: String,
            key: String,
            name: String,
            html_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableCommunityHealthFile {
            url: String,
            html_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CommunityProfileFiles {
            code_of_conduct: crate::components::schemas::NullableCodeOfConductSimple,
            code_of_conduct_file: crate::components::schemas::NullableCommunityHealthFile,
            license: crate::components::schemas::NullableLicenseSimple,
            contributing: crate::components::schemas::NullableCommunityHealthFile,
            readme: crate::components::schemas::NullableCommunityHealthFile,
            issue_template: crate::components::schemas::NullableCommunityHealthFile,
            pull_request_template: crate::components::schemas::NullableCommunityHealthFile,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CommunityProfile {
            health_percentage: i64,
            description: Option<String>,
            documentation: Option<String>,
            files: CommunityProfileFiles,
            updated_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content_reports_enabled: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CommitComparison {
            url: String,
            html_url: String,
            permalink_url: String,
            diff_url: String,
            patch_url: String,
            base_commit: crate::components::schemas::Commit,
            merge_base_commit: crate::components::schemas::Commit,
            status: String,
            ahead_by: i64,
            behind_by: i64,
            total_commits: i64,
            commits: Vec<crate::components::schemas::Commit>,
            #[serde(skip_serializing_if = "Option::is_none")]
            files: Option<Vec<crate::components::schemas::DiffEntry>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ContentReferenceAttachment {
            id: i64,
            title: String,
            body: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ContentTreeEntriesLinks {
            git: Option<String>,
            html: Option<String>,
            self: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ContentTreeEntries {
            type: String,
            size: i64,
            name: String,
            path: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            content: Option<String>,
            sha: String,
            url: String,
            git_url: Option<String>,
            html_url: Option<String>,
            download_url: Option<String>,
            _links: ContentTreeEntriesLinks,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ContentTreeLinks {
            git: Option<String>,
            html: Option<String>,
            self: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ContentTree {
            type: String,
            size: i64,
            name: String,
            path: String,
            sha: String,
            url: String,
            git_url: Option<String>,
            html_url: Option<String>,
            download_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            entries: Option<Vec<ContentTreeEntries>>,
            _links: ContentTreeLinks,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ContentFileLinks {
            git: Option<String>,
            html: Option<String>,
            self: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ContentFile {
            type: String,
            encoding: String,
            size: i64,
            name: String,
            path: String,
            content: String,
            sha: String,
            url: String,
            git_url: Option<String>,
            html_url: Option<String>,
            download_url: Option<String>,
            _links: ContentFileLinks,
            #[serde(skip_serializing_if = "Option::is_none")]
            target: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            submodule_git_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ContentSymlinkLinks {
            git: Option<String>,
            html: Option<String>,
            self: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ContentSymlink {
            type: String,
            target: String,
            size: i64,
            name: String,
            path: String,
            sha: String,
            url: String,
            git_url: Option<String>,
            html_url: Option<String>,
            download_url: Option<String>,
            _links: ContentSymlinkLinks,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ContentSubmoduleLinks {
            git: Option<String>,
            html: Option<String>,
            self: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ContentSubmodule {
            type: String,
            submodule_git_url: String,
            size: i64,
            name: String,
            path: String,
            sha: String,
            url: String,
            git_url: Option<String>,
            html_url: Option<String>,
            download_url: Option<String>,
            _links: ContentSubmoduleLinks,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct FileCommitContentLinks {
            #[serde(skip_serializing_if = "Option::is_none")]
            self: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            git: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct FileCommitContent {
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            path: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sha: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            size: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            git_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            download_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            _links: Option<FileCommitContentLinks>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct FileCommitCommitAuthor {
            #[serde(skip_serializing_if = "Option::is_none")]
            date: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            email: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct FileCommitCommitCommitter {
            #[serde(skip_serializing_if = "Option::is_none")]
            date: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            email: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct FileCommitCommitTree {
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sha: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct FileCommitCommitParents {
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sha: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct FileCommitCommitVerification {
            #[serde(skip_serializing_if = "Option::is_none")]
            verified: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            reason: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            signature: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            payload: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct FileCommitCommit {
            #[serde(skip_serializing_if = "Option::is_none")]
            sha: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            author: Option<FileCommitCommitAuthor>,
            #[serde(skip_serializing_if = "Option::is_none")]
            committer: Option<FileCommitCommitCommitter>,
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            tree: Option<FileCommitCommitTree>,
            #[serde(skip_serializing_if = "Option::is_none")]
            parents: Option<Vec<FileCommitCommitParents>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            verification: Option<FileCommitCommitVerification>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct FileCommit {
            content: Option<FileCommitContent>,
            commit: FileCommitCommit,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Contributor {
            #[serde(skip_serializing_if = "Option::is_none")]
            login: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            avatar_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            gravatar_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            followers_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            following_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            gists_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            starred_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subscriptions_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            organizations_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repos_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            events_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            received_events_url: Option<String>,
            type: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            site_admin: Option<bool>,
            contributions: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            email: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct DeploymentStatus {
            url: String,
            id: i64,
            node_id: String,
            state: String,
            creator: crate::components::schemas::NullableSimpleUser,
            description: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            environment: Option<String>,
            target_url: String,
            created_at: String,
            updated_at: String,
            deployment_url: String,
            repository_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            environment_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            log_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            performed_via_github_app: Option<crate::components::schemas::NullableIntegration>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct DeploymentBranchPolicy {
            protected_branches: bool,
            custom_branch_policies: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Environment {
            id: i64,
            node_id: String,
            name: String,
            url: String,
            html_url: String,
            created_at: String,
            updated_at: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            protection_rules: Option<Vec<>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            deployment_branch_policy: Option<crate::components::schemas::DeploymentBranchPolicy>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ShortBlob {
            url: String,
            sha: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Blob {
            content: String,
            encoding: String,
            url: String,
            sha: String,
            size: Option<i64>,
            node_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            highlighted_content: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GitCommitAuthor {
            date: String,
            email: String,
            name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GitCommitCommitter {
            date: String,
            email: String,
            name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GitCommitTree {
            sha: String,
            url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GitCommitParents {
            sha: String,
            url: String,
            html_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GitCommitVerification {
            verified: bool,
            reason: String,
            signature: Option<String>,
            payload: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GitCommit {
            sha: String,
            node_id: String,
            url: String,
            author: GitCommitAuthor,
            committer: GitCommitCommitter,
            message: String,
            tree: GitCommitTree,
            parents: Vec<GitCommitParents>,
            verification: GitCommitVerification,
            html_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GitRefObject {
            type: String,
            sha: String,
            url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GitRef {
            ref: String,
            node_id: String,
            url: String,
            object: GitRefObject,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GitTagTagger {
            date: String,
            email: String,
            name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GitTagObject {
            sha: String,
            type: String,
            url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GitTag {
            node_id: String,
            tag: String,
            sha: String,
            url: String,
            message: String,
            tagger: GitTagTagger,
            object: GitTagObject,
            #[serde(skip_serializing_if = "Option::is_none")]
            verification: Option<crate::components::schemas::Verification>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GitTreeTree {
            #[serde(skip_serializing_if = "Option::is_none")]
            path: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            mode: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sha: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            size: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GitTree {
            sha: String,
            url: String,
            truncated: bool,
            tree: Vec<GitTreeTree>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct HookResponse {
            code: Option<i64>,
            status: Option<String>,
            message: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct HookConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            email: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            password: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            room: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subdomain: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<crate::components::schemas::WebhookConfigUrl>,
            #[serde(skip_serializing_if = "Option::is_none")]
            insecure_ssl: Option<crate::components::schemas::WebhookConfigInsecureSsl>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content_type: Option<crate::components::schemas::WebhookConfigContentType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            digest: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            secret: Option<crate::components::schemas::WebhookConfigSecret>,
            #[serde(skip_serializing_if = "Option::is_none")]
            token: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Hook {
            type: String,
            id: i64,
            name: String,
            active: bool,
            events: Vec<String>,
            config: HookConfig,
            updated_at: String,
            created_at: String,
            url: String,
            test_url: String,
            ping_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            deliveries_url: Option<String>,
            last_response: crate::components::schemas::HookResponse,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ImportProjectChoices {
            #[serde(skip_serializing_if = "Option::is_none")]
            vcs: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            tfvc_project: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            human_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Import {
            vcs: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            use_lfs: Option<bool>,
            vcs_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            svc_root: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            tfvc_project: Option<String>,
            status: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            status_text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            failed_step: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            error_message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            import_percent: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            commit_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            push_percent: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_large_files: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            large_files_size: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            large_files_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            project_choices: Option<Vec<ImportProjectChoices>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            authors_count: Option<i64>,
            url: String,
            html_url: String,
            authors_url: String,
            repository_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            svn_root: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PorterAuthor {
            id: i64,
            remote_id: String,
            remote_name: String,
            email: String,
            name: String,
            url: String,
            import_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PorterLargeFile {
            ref_name: String,
            path: String,
            oid: String,
            size: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableIssueLabels {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            color: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            default: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        enum NullableIssueLabelsOneOf {
            String(String),
            NullableIssueLabels(NullableIssueLabels),
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableIssuePullRequest {
            #[serde(skip_serializing_if = "Option::is_none")]
            merged_at: Option<String>,
            diff_url: Option<String>,
            html_url: Option<String>,
            patch_url: Option<String>,
            url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NullableIssue {
            id: i64,
            node_id: String,
            url: String,
            repository_url: String,
            labels_url: String,
            comments_url: String,
            events_url: String,
            html_url: String,
            number: i64,
            state: String,
            title: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            body: Option<String>,
            user: crate::components::schemas::NullableSimpleUser,
            labels: Vec<NullableIssueLabelsOneOf>,
            assignee: crate::components::schemas::NullableSimpleUser,
            #[serde(skip_serializing_if = "Option::is_none")]
            assignees: Option<Vec<crate::components::schemas::SimpleUser>>,
            milestone: crate::components::schemas::NullableMilestone,
            locked: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            active_lock_reason: Option<String>,
            comments: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            pull_request: Option<NullableIssuePullRequest>,
            closed_at: Option<String>,
            created_at: String,
            updated_at: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            closed_by: Option<crate::components::schemas::NullableSimpleUser>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_html: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            timeline_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repository: Option<crate::components::schemas::Repository>,
            #[serde(skip_serializing_if = "Option::is_none")]
            performed_via_github_app: Option<crate::components::schemas::NullableIntegration>,
            author_association: crate::components::schemas::AuthorAssociation,
            #[serde(skip_serializing_if = "Option::is_none")]
            reactions: Option<crate::components::schemas::ReactionRollup>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct IssueEventLabel {
            name: Option<String>,
            color: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct IssueEventDismissedReview {
            state: String,
            review_id: i64,
            dismissal_message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            dismissal_commit_id: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct IssueEventMilestone {
            title: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct IssueEventProjectCard {
            url: String,
            id: i64,
            project_url: String,
            project_id: i64,
            column_name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            previous_column_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct IssueEventRename {
            from: String,
            to: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct IssueEvent {
            id: i64,
            node_id: String,
            url: String,
            actor: crate::components::schemas::NullableSimpleUser,
            event: String,
            commit_id: Option<String>,
            commit_url: Option<String>,
            created_at: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            issue: Option<crate::components::schemas::NullableIssue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            label: Option<crate::components::schemas::IssueEventLabel>,
            #[serde(skip_serializing_if = "Option::is_none")]
            assignee: Option<crate::components::schemas::NullableSimpleUser>,
            #[serde(skip_serializing_if = "Option::is_none")]
            assigner: Option<crate::components::schemas::NullableSimpleUser>,
            #[serde(skip_serializing_if = "Option::is_none")]
            review_requester: Option<crate::components::schemas::NullableSimpleUser>,
            #[serde(skip_serializing_if = "Option::is_none")]
            requested_reviewer: Option<crate::components::schemas::NullableSimpleUser>,
            #[serde(skip_serializing_if = "Option::is_none")]
            requested_team: Option<crate::components::schemas::Team>,
            #[serde(skip_serializing_if = "Option::is_none")]
            dismissed_review: Option<crate::components::schemas::IssueEventDismissedReview>,
            #[serde(skip_serializing_if = "Option::is_none")]
            milestone: Option<crate::components::schemas::IssueEventMilestone>,
            #[serde(skip_serializing_if = "Option::is_none")]
            project_card: Option<crate::components::schemas::IssueEventProjectCard>,
            #[serde(skip_serializing_if = "Option::is_none")]
            rename: Option<crate::components::schemas::IssueEventRename>,
            #[serde(skip_serializing_if = "Option::is_none")]
            author_association: Option<crate::components::schemas::AuthorAssociation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            lock_reason: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            performed_via_github_app: Option<crate::components::schemas::NullableIntegration>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct LabeledIssueEventLabel {
            name: String,
            color: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct LabeledIssueEvent {
            id: i64,
            node_id: String,
            url: String,
            actor: crate::components::schemas::SimpleUser,
            event: String,
            commit_id: Option<String>,
            commit_url: Option<String>,
            created_at: String,
            performed_via_github_app: crate::components::schemas::NullableIntegration,
            label: LabeledIssueEventLabel,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct UnlabeledIssueEventLabel {
            name: String,
            color: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct UnlabeledIssueEvent {
            id: i64,
            node_id: String,
            url: String,
            actor: crate::components::schemas::SimpleUser,
            event: String,
            commit_id: Option<String>,
            commit_url: Option<String>,
            created_at: String,
            performed_via_github_app: crate::components::schemas::NullableIntegration,
            label: UnlabeledIssueEventLabel,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct AssignedIssueEvent {
            id: i64,
            node_id: String,
            url: String,
            actor: crate::components::schemas::SimpleUser,
            event: String,
            commit_id: Option<String>,
            commit_url: Option<String>,
            created_at: String,
            performed_via_github_app: crate::components::schemas::Integration,
            assignee: crate::components::schemas::SimpleUser,
            assigner: crate::components::schemas::SimpleUser,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct UnassignedIssueEvent {
            id: i64,
            node_id: String,
            url: String,
            actor: crate::components::schemas::SimpleUser,
            event: String,
            commit_id: Option<String>,
            commit_url: Option<String>,
            created_at: String,
            performed_via_github_app: crate::components::schemas::NullableIntegration,
            assignee: crate::components::schemas::SimpleUser,
            assigner: crate::components::schemas::SimpleUser,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct MilestonedIssueEventMilestone {
            title: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct MilestonedIssueEvent {
            id: i64,
            node_id: String,
            url: String,
            actor: crate::components::schemas::SimpleUser,
            event: String,
            commit_id: Option<String>,
            commit_url: Option<String>,
            created_at: String,
            performed_via_github_app: crate::components::schemas::NullableIntegration,
            milestone: MilestonedIssueEventMilestone,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct DemilestonedIssueEventMilestone {
            title: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct DemilestonedIssueEvent {
            id: i64,
            node_id: String,
            url: String,
            actor: crate::components::schemas::SimpleUser,
            event: String,
            commit_id: Option<String>,
            commit_url: Option<String>,
            created_at: String,
            performed_via_github_app: crate::components::schemas::NullableIntegration,
            milestone: DemilestonedIssueEventMilestone,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RenamedIssueEventRename {
            from: String,
            to: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RenamedIssueEvent {
            id: i64,
            node_id: String,
            url: String,
            actor: crate::components::schemas::SimpleUser,
            event: String,
            commit_id: Option<String>,
            commit_url: Option<String>,
            created_at: String,
            performed_via_github_app: crate::components::schemas::NullableIntegration,
            rename: RenamedIssueEventRename,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ReviewRequestedIssueEvent {
            id: i64,
            node_id: String,
            url: String,
            actor: crate::components::schemas::SimpleUser,
            event: String,
            commit_id: Option<String>,
            commit_url: Option<String>,
            created_at: String,
            performed_via_github_app: crate::components::schemas::NullableIntegration,
            review_requester: crate::components::schemas::SimpleUser,
            #[serde(skip_serializing_if = "Option::is_none")]
            requested_team: Option<crate::components::schemas::Team>,
            #[serde(skip_serializing_if = "Option::is_none")]
            requested_reviewer: Option<crate::components::schemas::SimpleUser>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ReviewRequestRemovedIssueEvent {
            id: i64,
            node_id: String,
            url: String,
            actor: crate::components::schemas::SimpleUser,
            event: String,
            commit_id: Option<String>,
            commit_url: Option<String>,
            created_at: String,
            performed_via_github_app: crate::components::schemas::NullableIntegration,
            review_requester: crate::components::schemas::SimpleUser,
            #[serde(skip_serializing_if = "Option::is_none")]
            requested_team: Option<crate::components::schemas::Team>,
            #[serde(skip_serializing_if = "Option::is_none")]
            requested_reviewer: Option<crate::components::schemas::SimpleUser>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ReviewDismissedIssueEventDismissedReview {
            state: String,
            review_id: i64,
            dismissal_message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            dismissal_commit_id: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ReviewDismissedIssueEvent {
            id: i64,
            node_id: String,
            url: String,
            actor: crate::components::schemas::SimpleUser,
            event: String,
            commit_id: Option<String>,
            commit_url: Option<String>,
            created_at: String,
            performed_via_github_app: crate::components::schemas::NullableIntegration,
            dismissed_review: ReviewDismissedIssueEventDismissedReview,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct LockedIssueEvent {
            id: i64,
            node_id: String,
            url: String,
            actor: crate::components::schemas::SimpleUser,
            event: String,
            commit_id: Option<String>,
            commit_url: Option<String>,
            created_at: String,
            performed_via_github_app: crate::components::schemas::NullableIntegration,
            lock_reason: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct AddedToProjectIssueEventProjectCard {
            id: i64,
            url: String,
            project_id: i64,
            project_url: String,
            column_name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            previous_column_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct AddedToProjectIssueEvent {
            id: i64,
            node_id: String,
            url: String,
            actor: crate::components::schemas::SimpleUser,
            event: String,
            commit_id: Option<String>,
            commit_url: Option<String>,
            created_at: String,
            performed_via_github_app: crate::components::schemas::NullableIntegration,
            #[serde(skip_serializing_if = "Option::is_none")]
            project_card: Option<AddedToProjectIssueEventProjectCard>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct MovedColumnInProjectIssueEventProjectCard {
            id: i64,
            url: String,
            project_id: i64,
            project_url: String,
            column_name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            previous_column_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct MovedColumnInProjectIssueEvent {
            id: i64,
            node_id: String,
            url: String,
            actor: crate::components::schemas::SimpleUser,
            event: String,
            commit_id: Option<String>,
            commit_url: Option<String>,
            created_at: String,
            performed_via_github_app: crate::components::schemas::NullableIntegration,
            #[serde(skip_serializing_if = "Option::is_none")]
            project_card: Option<MovedColumnInProjectIssueEventProjectCard>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RemovedFromProjectIssueEventProjectCard {
            id: i64,
            url: String,
            project_id: i64,
            project_url: String,
            column_name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            previous_column_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RemovedFromProjectIssueEvent {
            id: i64,
            node_id: String,
            url: String,
            actor: crate::components::schemas::SimpleUser,
            event: String,
            commit_id: Option<String>,
            commit_url: Option<String>,
            created_at: String,
            performed_via_github_app: crate::components::schemas::NullableIntegration,
            #[serde(skip_serializing_if = "Option::is_none")]
            project_card: Option<RemovedFromProjectIssueEventProjectCard>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ConvertedNoteToIssueIssueEventProjectCard {
            id: i64,
            url: String,
            project_id: i64,
            project_url: String,
            column_name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            previous_column_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ConvertedNoteToIssueIssueEvent {
            id: i64,
            node_id: String,
            url: String,
            actor: crate::components::schemas::SimpleUser,
            event: String,
            commit_id: Option<String>,
            commit_url: Option<String>,
            created_at: String,
            performed_via_github_app: crate::components::schemas::Integration,
            #[serde(skip_serializing_if = "Option::is_none")]
            project_card: Option<ConvertedNoteToIssueIssueEventProjectCard>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Label {
            id: i64,
            node_id: String,
            url: String,
            name: String,
            description: Option<String>,
            color: String,
            default: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TimelineCommentEvent {
            event: String,
            actor: crate::components::schemas::SimpleUser,
            id: i64,
            node_id: String,
            url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            body: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_html: Option<String>,
            html_url: String,
            user: crate::components::schemas::SimpleUser,
            created_at: String,
            updated_at: String,
            issue_url: String,
            author_association: crate::components::schemas::AuthorAssociation,
            #[serde(skip_serializing_if = "Option::is_none")]
            performed_via_github_app: Option<crate::components::schemas::NullableIntegration>,
            #[serde(skip_serializing_if = "Option::is_none")]
            reactions: Option<crate::components::schemas::ReactionRollup>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TimelineCrossReferencedEventSource {
            #[serde(skip_serializing_if = "Option::is_none")]
            type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            issue: Option<crate::components::schemas::Issue>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TimelineCrossReferencedEvent {
            event: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            actor: Option<crate::components::schemas::SimpleUser>,
            created_at: String,
            updated_at: String,
            source: TimelineCrossReferencedEventSource,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TimelineCommittedEventAuthor {
            date: String,
            email: String,
            name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TimelineCommittedEventCommitter {
            date: String,
            email: String,
            name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TimelineCommittedEventTree {
            sha: String,
            url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TimelineCommittedEventParents {
            sha: String,
            url: String,
            html_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TimelineCommittedEventVerification {
            verified: bool,
            reason: String,
            signature: Option<String>,
            payload: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TimelineCommittedEvent {
            #[serde(skip_serializing_if = "Option::is_none")]
            event: Option<String>,
            sha: String,
            node_id: String,
            url: String,
            author: TimelineCommittedEventAuthor,
            committer: TimelineCommittedEventCommitter,
            message: String,
            tree: TimelineCommittedEventTree,
            parents: Vec<TimelineCommittedEventParents>,
            verification: TimelineCommittedEventVerification,
            html_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TimelineReviewedEventLinksHtml {
            href: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TimelineReviewedEventLinksPullRequest {
            href: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TimelineReviewedEventLinks {
            html: TimelineReviewedEventLinksHtml,
            pull_request: TimelineReviewedEventLinksPullRequest,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TimelineReviewedEvent {
            event: String,
            id: i64,
            node_id: String,
            user: crate::components::schemas::SimpleUser,
            body: Option<String>,
            state: String,
            html_url: String,
            pull_request_url: String,
            _links: TimelineReviewedEventLinks,
            #[serde(skip_serializing_if = "Option::is_none")]
            submitted_at: Option<String>,
            commit_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_html: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_text: Option<String>,
            author_association: crate::components::schemas::AuthorAssociation,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestReviewCommentLinksSelf {
            href: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestReviewCommentLinksHtml {
            href: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestReviewCommentLinksPullRequest {
            href: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestReviewCommentLinks {
            self: PullRequestReviewCommentLinksSelf,
            html: PullRequestReviewCommentLinksHtml,
            pull_request: PullRequestReviewCommentLinksPullRequest,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestReviewComment {
            url: String,
            pull_request_review_id: Option<i64>,
            id: i64,
            node_id: String,
            diff_hunk: String,
            path: String,
            position: i64,
            original_position: i64,
            commit_id: String,
            original_commit_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            in_reply_to_id: Option<i64>,
            user: crate::components::schemas::SimpleUser,
            body: String,
            created_at: String,
            updated_at: String,
            html_url: String,
            pull_request_url: String,
            author_association: crate::components::schemas::AuthorAssociation,
            _links: PullRequestReviewCommentLinks,
            #[serde(skip_serializing_if = "Option::is_none")]
            start_line: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            original_start_line: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            start_side: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            line: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            original_line: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            side: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            reactions: Option<crate::components::schemas::ReactionRollup>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_html: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_text: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TimelineLineCommentedEvent {
            #[serde(skip_serializing_if = "Option::is_none")]
            event: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            comments: Option<Vec<crate::components::schemas::PullRequestReviewComment>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TimelineCommitCommentedEvent {
            #[serde(skip_serializing_if = "Option::is_none")]
            event: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            commit_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            comments: Option<Vec<crate::components::schemas::CommitComment>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TimelineAssignedIssueEvent {
            id: i64,
            node_id: String,
            url: String,
            actor: crate::components::schemas::SimpleUser,
            event: String,
            commit_id: Option<String>,
            commit_url: Option<String>,
            created_at: String,
            performed_via_github_app: crate::components::schemas::NullableIntegration,
            assignee: crate::components::schemas::SimpleUser,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TimelineUnassignedIssueEvent {
            id: i64,
            node_id: String,
            url: String,
            actor: crate::components::schemas::SimpleUser,
            event: String,
            commit_id: Option<String>,
            commit_url: Option<String>,
            created_at: String,
            performed_via_github_app: crate::components::schemas::NullableIntegration,
            assignee: crate::components::schemas::SimpleUser,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TimelineIssueEvents;

        #[derive(Debug, Serialize, Deserialize)]
        struct DeployKey {
            id: i64,
            key: String,
            url: String,
            title: String,
            verified: bool,
            created_at: String,
            read_only: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Language;

        #[derive(Debug, Serialize, Deserialize)]
        struct LicenseContentLinks {
            git: Option<String>,
            html: Option<String>,
            self: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct LicenseContent {
            name: String,
            path: String,
            sha: String,
            size: i64,
            url: String,
            html_url: Option<String>,
            git_url: Option<String>,
            download_url: Option<String>,
            type: String,
            content: String,
            encoding: String,
            _links: LicenseContentLinks,
            license: crate::components::schemas::NullableLicenseSimple,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct MergedUpstream {
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            merge_type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            base_branch: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Milestone {
            url: String,
            html_url: String,
            labels_url: String,
            id: i64,
            node_id: String,
            number: i64,
            state: String,
            title: String,
            description: Option<String>,
            creator: crate::components::schemas::NullableSimpleUser,
            open_issues: i64,
            closed_issues: i64,
            created_at: String,
            updated_at: String,
            closed_at: Option<String>,
            due_on: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PagesSourceHash {
            branch: String,
            path: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PagesHttpsCertificate {
            state: String,
            description: String,
            domains: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            expires_at: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Page {
            url: String,
            status: Option<String>,
            cname: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            protected_domain_state: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pending_domain_unverified_at: Option<String>,
            custom_404: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            source: Option<crate::components::schemas::PagesSourceHash>,
            public: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            https_certificate: Option<crate::components::schemas::PagesHttpsCertificate>,
            #[serde(skip_serializing_if = "Option::is_none")]
            https_enforced: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PageBuildError {
            message: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PageBuild {
            url: String,
            status: String,
            error: PageBuildError,
            pusher: crate::components::schemas::NullableSimpleUser,
            commit: String,
            duration: i64,
            created_at: String,
            updated_at: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PageBuildStatus {
            url: String,
            status: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PagesHealthCheckDomain {
            #[serde(skip_serializing_if = "Option::is_none")]
            host: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            uri: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            nameservers: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            dns_resolves: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_proxied: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_cloudflare_ip: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_fastly_ip: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_old_ip_address: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_a_record: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_cname_record: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_mx_records_present: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_valid_domain: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_apex_domain: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            should_be_a_record: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_cname_to_github_user_domain: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_cname_to_pages_dot_github_dot_com: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_cname_to_fastly: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_pointed_to_github_pages_ip: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_non_github_pages_ip_present: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_pages_domain: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_served_by_pages: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_valid: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            reason: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            responds_to_https: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            enforces_https: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            https_error: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_https_eligible: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            caa_error: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PagesHealthCheckAltDomain {
            #[serde(skip_serializing_if = "Option::is_none")]
            host: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            uri: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            nameservers: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            dns_resolves: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_proxied: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_cloudflare_ip: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_fastly_ip: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_old_ip_address: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_a_record: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_cname_record: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_mx_records_present: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_valid_domain: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_apex_domain: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            should_be_a_record: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_cname_to_github_user_domain: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_cname_to_pages_dot_github_dot_com: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_cname_to_fastly: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_pointed_to_github_pages_ip: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_non_github_pages_ip_present: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_pages_domain: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_served_by_pages: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_valid: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            reason: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            responds_to_https: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            enforces_https: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            https_error: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_https_eligible: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            caa_error: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PagesHealthCheck {
            #[serde(skip_serializing_if = "Option::is_none")]
            domain: Option<PagesHealthCheckDomain>,
            #[serde(skip_serializing_if = "Option::is_none")]
            alt_domain: Option<PagesHealthCheckAltDomain>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TeamSimple {
            id: i64,
            node_id: String,
            url: String,
            members_url: String,
            name: String,
            description: Option<String>,
            permission: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            privacy: Option<String>,
            html_url: String,
            repositories_url: String,
            slug: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            ldap_dn: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestLabels {
            id: i64,
            node_id: String,
            url: String,
            name: String,
            description: Option<String>,
            color: String,
            default: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestHeadRepoOwner {
            avatar_url: String,
            events_url: String,
            followers_url: String,
            following_url: String,
            gists_url: String,
            gravatar_id: Option<String>,
            html_url: String,
            id: i64,
            node_id: String,
            login: String,
            organizations_url: String,
            received_events_url: String,
            repos_url: String,
            site_admin: bool,
            starred_url: String,
            subscriptions_url: String,
            type: String,
            url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestHeadRepoPermissions {
            admin: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            maintain: Option<bool>,
            push: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            triage: Option<bool>,
            pull: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestHeadRepoLicense {
            key: String,
            name: String,
            url: Option<String>,
            spdx_id: Option<String>,
            node_id: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestHeadRepo {
            archive_url: String,
            assignees_url: String,
            blobs_url: String,
            branches_url: String,
            collaborators_url: String,
            comments_url: String,
            commits_url: String,
            compare_url: String,
            contents_url: String,
            contributors_url: String,
            deployments_url: String,
            description: Option<String>,
            downloads_url: String,
            events_url: String,
            fork: bool,
            forks_url: String,
            full_name: String,
            git_commits_url: String,
            git_refs_url: String,
            git_tags_url: String,
            hooks_url: String,
            html_url: String,
            id: i64,
            node_id: String,
            issue_comment_url: String,
            issue_events_url: String,
            issues_url: String,
            keys_url: String,
            labels_url: String,
            languages_url: String,
            merges_url: String,
            milestones_url: String,
            name: String,
            notifications_url: String,
            owner: PullRequestHeadRepoOwner,
            private: bool,
            pulls_url: String,
            releases_url: String,
            stargazers_url: String,
            statuses_url: String,
            subscribers_url: String,
            subscription_url: String,
            tags_url: String,
            teams_url: String,
            trees_url: String,
            url: String,
            clone_url: String,
            default_branch: String,
            forks: i64,
            forks_count: i64,
            git_url: String,
            has_downloads: bool,
            has_issues: bool,
            has_projects: bool,
            has_wiki: bool,
            has_pages: bool,
            homepage: Option<String>,
            language: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            master_branch: Option<String>,
            archived: bool,
            disabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            visibility: Option<String>,
            mirror_url: Option<String>,
            open_issues: i64,
            open_issues_count: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<PullRequestHeadRepoPermissions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            temp_clone_token: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_merge_commit: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_squash_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_rebase_merge: Option<bool>,
            license: Option<PullRequestHeadRepoLicense>,
            pushed_at: String,
            size: i64,
            ssh_url: String,
            stargazers_count: i64,
            svn_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            topics: Option<Vec<String>>,
            watchers: i64,
            watchers_count: i64,
            created_at: String,
            updated_at: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_forking: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_template: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestHeadUser {
            avatar_url: String,
            events_url: String,
            followers_url: String,
            following_url: String,
            gists_url: String,
            gravatar_id: Option<String>,
            html_url: String,
            id: i64,
            node_id: String,
            login: String,
            organizations_url: String,
            received_events_url: String,
            repos_url: String,
            site_admin: bool,
            starred_url: String,
            subscriptions_url: String,
            type: String,
            url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestHead {
            label: String,
            ref: String,
            repo: Option<PullRequestHeadRepo>,
            sha: String,
            user: PullRequestHeadUser,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestBaseRepoOwner {
            avatar_url: String,
            events_url: String,
            followers_url: String,
            following_url: String,
            gists_url: String,
            gravatar_id: Option<String>,
            html_url: String,
            id: i64,
            node_id: String,
            login: String,
            organizations_url: String,
            received_events_url: String,
            repos_url: String,
            site_admin: bool,
            starred_url: String,
            subscriptions_url: String,
            type: String,
            url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestBaseRepoPermissions {
            admin: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            maintain: Option<bool>,
            push: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            triage: Option<bool>,
            pull: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestBaseRepo {
            archive_url: String,
            assignees_url: String,
            blobs_url: String,
            branches_url: String,
            collaborators_url: String,
            comments_url: String,
            commits_url: String,
            compare_url: String,
            contents_url: String,
            contributors_url: String,
            deployments_url: String,
            description: Option<String>,
            downloads_url: String,
            events_url: String,
            fork: bool,
            forks_url: String,
            full_name: String,
            git_commits_url: String,
            git_refs_url: String,
            git_tags_url: String,
            hooks_url: String,
            html_url: String,
            id: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_template: Option<bool>,
            node_id: String,
            issue_comment_url: String,
            issue_events_url: String,
            issues_url: String,
            keys_url: String,
            labels_url: String,
            languages_url: String,
            merges_url: String,
            milestones_url: String,
            name: String,
            notifications_url: String,
            owner: PullRequestBaseRepoOwner,
            private: bool,
            pulls_url: String,
            releases_url: String,
            stargazers_url: String,
            statuses_url: String,
            subscribers_url: String,
            subscription_url: String,
            tags_url: String,
            teams_url: String,
            trees_url: String,
            url: String,
            clone_url: String,
            default_branch: String,
            forks: i64,
            forks_count: i64,
            git_url: String,
            has_downloads: bool,
            has_issues: bool,
            has_projects: bool,
            has_wiki: bool,
            has_pages: bool,
            homepage: Option<String>,
            language: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            master_branch: Option<String>,
            archived: bool,
            disabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            visibility: Option<String>,
            mirror_url: Option<String>,
            open_issues: i64,
            open_issues_count: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<PullRequestBaseRepoPermissions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            temp_clone_token: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_merge_commit: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_squash_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_rebase_merge: Option<bool>,
            license: crate::components::schemas::NullableLicenseSimple,
            pushed_at: String,
            size: i64,
            ssh_url: String,
            stargazers_count: i64,
            svn_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            topics: Option<Vec<String>>,
            watchers: i64,
            watchers_count: i64,
            created_at: String,
            updated_at: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_forking: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestBaseUser {
            avatar_url: String,
            events_url: String,
            followers_url: String,
            following_url: String,
            gists_url: String,
            gravatar_id: Option<String>,
            html_url: String,
            id: i64,
            node_id: String,
            login: String,
            organizations_url: String,
            received_events_url: String,
            repos_url: String,
            site_admin: bool,
            starred_url: String,
            subscriptions_url: String,
            type: String,
            url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestBase {
            label: String,
            ref: String,
            repo: PullRequestBaseRepo,
            sha: String,
            user: PullRequestBaseUser,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestLinks {
            comments: crate::components::schemas::Link,
            commits: crate::components::schemas::Link,
            statuses: crate::components::schemas::Link,
            html: crate::components::schemas::Link,
            issue: crate::components::schemas::Link,
            review_comments: crate::components::schemas::Link,
            review_comment: crate::components::schemas::Link,
            self: crate::components::schemas::Link,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequest {
            url: String,
            id: i64,
            node_id: String,
            html_url: String,
            diff_url: String,
            patch_url: String,
            issue_url: String,
            commits_url: String,
            review_comments_url: String,
            review_comment_url: String,
            comments_url: String,
            statuses_url: String,
            number: i64,
            state: String,
            locked: bool,
            title: String,
            user: crate::components::schemas::NullableSimpleUser,
            body: Option<String>,
            labels: Vec<PullRequestLabels>,
            milestone: crate::components::schemas::NullableMilestone,
            #[serde(skip_serializing_if = "Option::is_none")]
            active_lock_reason: Option<String>,
            created_at: String,
            updated_at: String,
            closed_at: Option<String>,
            merged_at: Option<String>,
            merge_commit_sha: Option<String>,
            assignee: crate::components::schemas::NullableSimpleUser,
            #[serde(skip_serializing_if = "Option::is_none")]
            assignees: Option<Vec<crate::components::schemas::SimpleUser>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            requested_reviewers: Option<Vec<crate::components::schemas::SimpleUser>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            requested_teams: Option<Vec<crate::components::schemas::TeamSimple>>,
            head: PullRequestHead,
            base: PullRequestBase,
            _links: PullRequestLinks,
            author_association: crate::components::schemas::AuthorAssociation,
            auto_merge: crate::components::schemas::AutoMerge,
            #[serde(skip_serializing_if = "Option::is_none")]
            draft: Option<bool>,
            merged: bool,
            mergeable: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            rebaseable: Option<bool>,
            mergeable_state: String,
            merged_by: crate::components::schemas::NullableSimpleUser,
            comments: i64,
            review_comments: i64,
            maintainer_can_modify: bool,
            commits: i64,
            additions: i64,
            deletions: i64,
            changed_files: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestMergeResult {
            sha: String,
            merged: bool,
            message: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestReviewRequest {
            users: Vec<crate::components::schemas::SimpleUser>,
            teams: Vec<crate::components::schemas::Team>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestReviewLinksHtml {
            href: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestReviewLinksPullRequest {
            href: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestReviewLinks {
            html: PullRequestReviewLinksHtml,
            pull_request: PullRequestReviewLinksPullRequest,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PullRequestReview {
            id: i64,
            node_id: String,
            user: crate::components::schemas::NullableSimpleUser,
            body: String,
            state: String,
            html_url: String,
            pull_request_url: String,
            _links: PullRequestReviewLinks,
            #[serde(skip_serializing_if = "Option::is_none")]
            submitted_at: Option<String>,
            commit_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_html: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_text: Option<String>,
            author_association: crate::components::schemas::AuthorAssociation,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ReviewCommentLinks {
            self: crate::components::schemas::Link,
            html: crate::components::schemas::Link,
            pull_request: crate::components::schemas::Link,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ReviewComment {
            url: String,
            pull_request_review_id: Option<i64>,
            id: i64,
            node_id: String,
            diff_hunk: String,
            path: String,
            position: Option<i64>,
            original_position: i64,
            commit_id: String,
            original_commit_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            in_reply_to_id: Option<i64>,
            user: crate::components::schemas::NullableSimpleUser,
            body: String,
            created_at: String,
            updated_at: String,
            html_url: String,
            pull_request_url: String,
            author_association: crate::components::schemas::AuthorAssociation,
            _links: ReviewCommentLinks,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_html: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            reactions: Option<crate::components::schemas::ReactionRollup>,
            #[serde(skip_serializing_if = "Option::is_none")]
            side: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            start_side: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            line: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            original_line: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            start_line: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            original_start_line: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ReleaseAsset {
            url: String,
            browser_download_url: String,
            id: i64,
            node_id: String,
            name: String,
            label: Option<String>,
            state: String,
            content_type: String,
            size: i64,
            download_count: i64,
            created_at: String,
            updated_at: String,
            uploader: crate::components::schemas::NullableSimpleUser,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Release {
            url: String,
            html_url: String,
            assets_url: String,
            upload_url: String,
            tarball_url: Option<String>,
            zipball_url: Option<String>,
            id: i64,
            node_id: String,
            tag_name: String,
            target_commitish: String,
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body: Option<String>,
            draft: bool,
            prerelease: bool,
            created_at: String,
            published_at: Option<String>,
            author: crate::components::schemas::SimpleUser,
            assets: Vec<crate::components::schemas::ReleaseAsset>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_html: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            mentions_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            discussion_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            reactions: Option<crate::components::schemas::ReactionRollup>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ReleaseNotesContent {
            name: String,
            body: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct SecretScanningAlert {
            #[serde(skip_serializing_if = "Option::is_none")]
            number: Option<crate::components::schemas::AlertNumber>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created_at: Option<crate::components::schemas::AlertCreatedAt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<crate::components::schemas::AlertUrl>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<crate::components::schemas::AlertHtmlUrl>,
            #[serde(skip_serializing_if = "Option::is_none")]
            locations_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            state: Option<crate::components::schemas::SecretScanningAlertState>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolution: Option<crate::components::schemas::SecretScanningAlertResolution>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolved_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolved_by: Option<crate::components::schemas::NullableSimpleUser>,
            #[serde(skip_serializing_if = "Option::is_none")]
            secret_type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            secret: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Stargazer {
            starred_at: String,
            user: crate::components::schemas::NullableSimpleUser,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CommitActivity {
            days: Vec<i64>,
            total: i64,
            week: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ContributorActivityWeeks {
            #[serde(skip_serializing_if = "Option::is_none")]
            w: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            a: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            d: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            c: Option<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ContributorActivity {
            author: crate::components::schemas::NullableSimpleUser,
            total: i64,
            weeks: Vec<ContributorActivityWeeks>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ParticipationStats {
            all: Vec<i64>,
            owner: Vec<i64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RepositorySubscription {
            subscribed: bool,
            ignored: bool,
            reason: Option<String>,
            created_at: String,
            url: String,
            repository_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TagCommit {
            sha: String,
            url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Tag {
            name: String,
            commit: TagCommit,
            zipball_url: String,
            tarball_url: String,
            node_id: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Topic {
            names: Vec<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Traffic {
            timestamp: String,
            uniques: i64,
            count: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CloneTraffic {
            count: i64,
            uniques: i64,
            clones: Vec<crate::components::schemas::Traffic>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ContentTraffic {
            path: String,
            title: String,
            count: i64,
            uniques: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ReferrerTraffic {
            referrer: String,
            count: i64,
            uniques: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ViewTraffic {
            count: i64,
            uniques: i64,
            views: Vec<crate::components::schemas::Traffic>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimGroupListEnterpriseResourcesMembers {
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            $ref: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            display: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimGroupListEnterpriseResourcesMeta {
            #[serde(skip_serializing_if = "Option::is_none")]
            resourceType: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            lastModified: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            location: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimGroupListEnterpriseResources {
            schemas: Vec<String>,
            id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            externalId: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            displayName: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            members: Option<Vec<ScimGroupListEnterpriseResourcesMembers>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            meta: Option<ScimGroupListEnterpriseResourcesMeta>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimGroupListEnterprise {
            schemas: Vec<String>,
            totalResults: i64,
            itemsPerPage: i64,
            startIndex: i64,
            Resources: Vec<ScimGroupListEnterpriseResources>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimEnterpriseGroupMembers {
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            $ref: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            display: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimEnterpriseGroupMeta {
            #[serde(skip_serializing_if = "Option::is_none")]
            resourceType: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            lastModified: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            location: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimEnterpriseGroup {
            schemas: Vec<String>,
            id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            externalId: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            displayName: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            members: Option<Vec<ScimEnterpriseGroupMembers>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            meta: Option<ScimEnterpriseGroupMeta>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimUserListEnterpriseResourcesName {
            #[serde(skip_serializing_if = "Option::is_none")]
            givenName: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            familyName: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimUserListEnterpriseResourcesEmails {
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            primary: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            type: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimUserListEnterpriseResourcesGroups {
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimUserListEnterpriseResourcesMeta {
            #[serde(skip_serializing_if = "Option::is_none")]
            resourceType: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            lastModified: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            location: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimUserListEnterpriseResources {
            schemas: Vec<String>,
            id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            externalId: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            userName: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<ScimUserListEnterpriseResourcesName>,
            #[serde(skip_serializing_if = "Option::is_none")]
            emails: Option<Vec<ScimUserListEnterpriseResourcesEmails>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            groups: Option<Vec<ScimUserListEnterpriseResourcesGroups>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            active: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            meta: Option<ScimUserListEnterpriseResourcesMeta>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimUserListEnterprise {
            schemas: Vec<String>,
            totalResults: i64,
            itemsPerPage: i64,
            startIndex: i64,
            Resources: Vec<ScimUserListEnterpriseResources>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimEnterpriseUserName {
            #[serde(skip_serializing_if = "Option::is_none")]
            givenName: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            familyName: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimEnterpriseUserEmails {
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            primary: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimEnterpriseUserGroups {
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimEnterpriseUserMeta {
            #[serde(skip_serializing_if = "Option::is_none")]
            resourceType: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            lastModified: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            location: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimEnterpriseUser {
            schemas: Vec<String>,
            id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            externalId: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            userName: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<ScimEnterpriseUserName>,
            #[serde(skip_serializing_if = "Option::is_none")]
            emails: Option<Vec<ScimEnterpriseUserEmails>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            groups: Option<Vec<ScimEnterpriseUserGroups>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            active: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            meta: Option<ScimEnterpriseUserMeta>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimUserName {
            givenName: Option<String>,
            familyName: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            formatted: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimUserEmails {
            value: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            primary: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimUserMeta {
            #[serde(skip_serializing_if = "Option::is_none")]
            resourceType: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            lastModified: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            location: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimUserOperationsValue;

        #[derive(Debug, Serialize, Deserialize)]
        enum ScimUserOperationsValueOneOf {
            String(String),
            ScimUserOperationsValue(ScimUserOperationsValue),
            Vec(Vec<>),
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimUserOperations {
            op: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            path: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<ScimUserOperationsValueOneOf>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimUser {
            schemas: Vec<String>,
            id: String,
            externalId: Option<String>,
            userName: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            displayName: Option<String>,
            name: ScimUserName,
            emails: Vec<ScimUserEmails>,
            active: bool,
            meta: ScimUserMeta,
            #[serde(skip_serializing_if = "Option::is_none")]
            organization_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            operations: Option<Vec<ScimUserOperations>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            groups: Option<Vec<>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ScimUserList {
            schemas: Vec<String>,
            totalResults: i64,
            itemsPerPage: i64,
            startIndex: i64,
            Resources: Vec<crate::components::schemas::ScimUser>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodeSearchResultItem {
            name: String,
            path: String,
            sha: String,
            url: String,
            git_url: String,
            html_url: String,
            repository: crate::components::schemas::MinimalRepository,
            score: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            file_size: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            language: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            last_modified_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            line_numbers: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            text_matches: Option<crate::components::schemas::SearchResultTextMatches>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CommitSearchResultItemCommitAuthor {
            name: String,
            email: String,
            date: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CommitSearchResultItemCommitTree {
            sha: String,
            url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CommitSearchResultItemCommit {
            author: CommitSearchResultItemCommitAuthor,
            committer: crate::components::schemas::NullableGitUser,
            comment_count: i64,
            message: String,
            tree: CommitSearchResultItemCommitTree,
            url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            verification: Option<crate::components::schemas::Verification>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CommitSearchResultItemParents {
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            html_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sha: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CommitSearchResultItem {
            url: String,
            sha: String,
            html_url: String,
            comments_url: String,
            commit: CommitSearchResultItemCommit,
            author: crate::components::schemas::NullableSimpleUser,
            committer: crate::components::schemas::NullableGitUser,
            parents: Vec<CommitSearchResultItemParents>,
            repository: crate::components::schemas::MinimalRepository,
            score: i64,
            node_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            text_matches: Option<crate::components::schemas::SearchResultTextMatches>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct IssueSearchResultItemLabels {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            color: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            default: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct IssueSearchResultItemPullRequest {
            #[serde(skip_serializing_if = "Option::is_none")]
            merged_at: Option<String>,
            diff_url: Option<String>,
            html_url: Option<String>,
            patch_url: Option<String>,
            url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct IssueSearchResultItem {
            url: String,
            repository_url: String,
            labels_url: String,
            comments_url: String,
            events_url: String,
            html_url: String,
            id: i64,
            node_id: String,
            number: i64,
            title: String,
            locked: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            active_lock_reason: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            assignees: Option<Vec<crate::components::schemas::SimpleUser>>,
            user: crate::components::schemas::NullableSimpleUser,
            labels: Vec<IssueSearchResultItemLabels>,
            state: String,
            assignee: crate::components::schemas::NullableSimpleUser,
            milestone: crate::components::schemas::NullableMilestone,
            comments: i64,
            created_at: String,
            updated_at: String,
            closed_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            text_matches: Option<crate::components::schemas::SearchResultTextMatches>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pull_request: Option<IssueSearchResultItemPullRequest>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body: Option<String>,
            score: i64,
            author_association: crate::components::schemas::AuthorAssociation,
            #[serde(skip_serializing_if = "Option::is_none")]
            draft: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            repository: Option<crate::components::schemas::Repository>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_html: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body_text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            timeline_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            performed_via_github_app: Option<crate::components::schemas::NullableIntegration>,
            #[serde(skip_serializing_if = "Option::is_none")]
            reactions: Option<crate::components::schemas::ReactionRollup>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct LabelSearchResultItem {
            id: i64,
            node_id: String,
            url: String,
            name: String,
            color: String,
            default: bool,
            description: Option<String>,
            score: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            text_matches: Option<crate::components::schemas::SearchResultTextMatches>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RepoSearchResultItemPermissions {
            admin: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            maintain: Option<bool>,
            push: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            triage: Option<bool>,
            pull: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct RepoSearchResultItem {
            id: i64,
            node_id: String,
            name: String,
            full_name: String,
            owner: crate::components::schemas::NullableSimpleUser,
            private: bool,
            html_url: String,
            description: Option<String>,
            fork: bool,
            url: String,
            created_at: String,
            updated_at: String,
            pushed_at: String,
            homepage: Option<String>,
            size: i64,
            stargazers_count: i64,
            watchers_count: i64,
            language: Option<String>,
            forks_count: i64,
            open_issues_count: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            master_branch: Option<String>,
            default_branch: String,
            score: i64,
            forks_url: String,
            keys_url: String,
            collaborators_url: String,
            teams_url: String,
            hooks_url: String,
            issue_events_url: String,
            events_url: String,
            assignees_url: String,
            branches_url: String,
            tags_url: String,
            blobs_url: String,
            git_tags_url: String,
            git_refs_url: String,
            trees_url: String,
            statuses_url: String,
            languages_url: String,
            stargazers_url: String,
            contributors_url: String,
            subscribers_url: String,
            subscription_url: String,
            commits_url: String,
            git_commits_url: String,
            comments_url: String,
            issue_comment_url: String,
            contents_url: String,
            compare_url: String,
            merges_url: String,
            archive_url: String,
            downloads_url: String,
            issues_url: String,
            pulls_url: String,
            milestones_url: String,
            notifications_url: String,
            labels_url: String,
            releases_url: String,
            deployments_url: String,
            git_url: String,
            ssh_url: String,
            clone_url: String,
            svn_url: String,
            forks: i64,
            open_issues: i64,
            watchers: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            topics: Option<Vec<String>>,
            mirror_url: Option<String>,
            has_issues: bool,
            has_projects: bool,
            has_pages: bool,
            has_wiki: bool,
            has_downloads: bool,
            archived: bool,
            disabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            visibility: Option<String>,
            license: crate::components::schemas::NullableLicenseSimple,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<RepoSearchResultItemPermissions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            text_matches: Option<crate::components::schemas::SearchResultTextMatches>,
            #[serde(skip_serializing_if = "Option::is_none")]
            temp_clone_token: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_merge_commit: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_squash_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_rebase_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_auto_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            delete_branch_on_merge: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allow_forking: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_template: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TopicSearchResultItemRelatedTopicRelation {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            topic_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            relation_type: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TopicSearchResultItemRelated {
            #[serde(skip_serializing_if = "Option::is_none")]
            topic_relation: Option<TopicSearchResultItemRelatedTopicRelation>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TopicSearchResultItemAliasesTopicRelation {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            topic_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            relation_type: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TopicSearchResultItemAliases {
            #[serde(skip_serializing_if = "Option::is_none")]
            topic_relation: Option<TopicSearchResultItemAliasesTopicRelation>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct TopicSearchResultItem {
            name: String,
            display_name: Option<String>,
            short_description: Option<String>,
            description: Option<String>,
            created_by: Option<String>,
            released: Option<String>,
            created_at: String,
            updated_at: String,
            featured: bool,
            curated: bool,
            score: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            repository_count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            logo_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            text_matches: Option<crate::components::schemas::SearchResultTextMatches>,
            #[serde(skip_serializing_if = "Option::is_none")]
            related: Option<Vec<TopicSearchResultItemRelated>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            aliases: Option<Vec<TopicSearchResultItemAliases>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct UserSearchResultItem {
            login: String,
            id: i64,
            node_id: String,
            avatar_url: String,
            gravatar_id: Option<String>,
            url: String,
            html_url: String,
            followers_url: String,
            subscriptions_url: String,
            organizations_url: String,
            repos_url: String,
            received_events_url: String,
            type: String,
            score: i64,
            following_url: String,
            gists_url: String,
            starred_url: String,
            events_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            public_repos: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            public_gists: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            followers: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            following: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            updated_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            bio: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            email: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            location: Option<String>,
            site_admin: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            hireable: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            text_matches: Option<crate::components::schemas::SearchResultTextMatches>,
            #[serde(skip_serializing_if = "Option::is_none")]
            blog: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            company: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            suspended_at: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PrivateUserPlan {
            collaborators: i64,
            name: String,
            space: i64,
            private_repos: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct PrivateUser {
            login: String,
            id: i64,
            node_id: String,
            avatar_url: String,
            gravatar_id: Option<String>,
            url: String,
            html_url: String,
            followers_url: String,
            following_url: String,
            gists_url: String,
            starred_url: String,
            subscriptions_url: String,
            organizations_url: String,
            repos_url: String,
            events_url: String,
            received_events_url: String,
            type: String,
            site_admin: bool,
            name: Option<String>,
            company: Option<String>,
            blog: Option<String>,
            location: Option<String>,
            email: Option<String>,
            hireable: Option<bool>,
            bio: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            twitter_username: Option<String>,
            public_repos: i64,
            public_gists: i64,
            followers: i64,
            following: i64,
            created_at: String,
            updated_at: String,
            private_gists: i64,
            total_private_repos: i64,
            owned_private_repos: i64,
            disk_usage: i64,
            collaborators: i64,
            two_factor_authentication: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            plan: Option<PrivateUserPlan>,
            #[serde(skip_serializing_if = "Option::is_none")]
            suspended_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            business_plus: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ldap_dn: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodespacesSecret {
            name: String,
            created_at: String,
            updated_at: String,
            visibility: String,
            selected_repositories_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CodespacesUserPublicKey {
            key_id: String,
            key: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Email {
            email: String,
            primary: bool,
            verified: bool,
            visibility: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GpgKeyEmails {
            #[serde(skip_serializing_if = "Option::is_none")]
            email: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            verified: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GpgKeySubkeys {
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            primary_key_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            key_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            public_key: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            emails: Option<Vec<>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subkeys: Option<Vec<>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            can_sign: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            can_encrypt_comms: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            can_encrypt_storage: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            can_certify: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            created_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            expires_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            raw_key: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct GpgKey {
            id: i64,
            primary_key_id: Option<i64>,
            key_id: String,
            public_key: String,
            emails: Vec<GpgKeyEmails>,
            subkeys: Vec<GpgKeySubkeys>,
            can_sign: bool,
            can_encrypt_comms: bool,
            can_encrypt_storage: bool,
            can_certify: bool,
            created_at: String,
            expires_at: Option<String>,
            raw_key: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Key {
            key: String,
            id: i64,
            url: String,
            title: String,
            created_at: String,
            verified: bool,
            read_only: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct MarketplaceAccount {
            url: String,
            id: i64,
            type: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            login: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            email: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            organization_billing_email: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct UserMarketplacePurchase {
            billing_cycle: String,
            next_billing_date: Option<String>,
            unit_count: Option<i64>,
            on_free_trial: bool,
            free_trial_ends_on: Option<String>,
            updated_at: Option<String>,
            account: crate::components::schemas::MarketplaceAccount,
            plan: crate::components::schemas::MarketplaceListingPlan,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct StarredRepository {
            starred_at: String,
            repo: crate::components::schemas::Repository,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct HovercardContexts {
            message: String,
            octicon: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Hovercard {
            contexts: Vec<HovercardContexts>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct KeySimple {
            id: i64,
            key: String,
        }
    }
}