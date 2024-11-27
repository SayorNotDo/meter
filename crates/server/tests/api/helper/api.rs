use super::result::AppResponseResult;
use crate::unwrap;
use anyhow::Ok;

use log_derive::logfn;
use reqwest::StatusCode;

use server::{
    configure::server::ConfigHTTP,
    constant::{HTTP, PROJECT_ID},
    dto::{
        request::{
            case::*,
            file::{CreateModuleRequest, DeleteModuleRequest, QueryModuleParam},
            user::{DeleteUserRequest, LoginRequest, UpdateUserStatusRequest},
            *,
        },
        response::{case::FunctionalCaseResponse, user::*, *},
    },
    entity::{
        case::Field,
        permission::Permission,
        user::{UserRole, UserRolePermission},
    },
    utils::http::HttpClientExt,
};

pub struct Api {
    addr: String,
}

impl Api {
    pub fn new(config: &ConfigHTTP) -> Self {
        Self {
            addr: config.get_http_addr(),
        }
    }

    #[logfn(Info)]
    pub async fn register(
        &self,
        token: &str,
        project_id: i32,
        req: &RegisterRequest,
    ) -> anyhow::Result<(StatusCode, AppResponseResult)> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );
        headers.append(PROJECT_ID, project_id.to_string().parse()?);
        let resp = HTTP
            .post(format!("{}/auth/register", self.addr))
            .headers(headers)
            .json(req)
            .send()
            .await?;
        Ok((resp.status(), resp.json().await?))
    }

    #[logfn(Info)]
    pub async fn login(
        &self,
        req: &LoginRequest,
    ) -> anyhow::Result<(StatusCode, AppResponseResult<LoginResponse>)> {
        let resp = HTTP
            .post_request(&format!("{}/auth/login", self.addr), req)
            .await?;
        Ok((resp.status(), resp.json().await?))
    }

    pub async fn get_token(&self, req: &LoginRequest) -> anyhow::Result<TokenResponse> {
        let (_, resp) = self.login(req).await?;
        let resp = unwrap!(resp);
        match resp {
            LoginResponse::Token(token) => Ok(token),
            LoginResponse::Code { .. } => Err(anyhow::anyhow!("Get token failed...")),
        }
    }

    #[logfn(Info)]
    pub async fn logout(&self, token: &str) -> anyhow::Result<(StatusCode, AppResponseResult)> {
        let resp = HTTP
            .get(format!("{}/auth/logout", self.addr))
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
            .send()
            .await?;
        Ok((resp.status(), resp.json().await?))
    }

    #[logfn(Info)]
    pub async fn create_role(
        &self,
        token: &str,
        project_id: i32,
        req: &CreateRoleRequest,
    ) -> anyhow::Result<(StatusCode, AppResponseResult<CreateEntityResponse>)> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );
        headers.append(PROJECT_ID, project_id.to_string().parse()?);
        let resp = HTTP
            .post(format!("{}/system/user/role", self.addr))
            .headers(headers)
            .json(req)
            .send()
            .await?;
        Ok((resp.status(), resp.json().await?))
    }

    #[logfn(Info)]
    pub async fn update_user_status(
        &self,
        token: &str,
        project_id: i32,
        req: &UpdateUserStatusRequest,
    ) -> anyhow::Result<(StatusCode, AppResponseResult)> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );
        headers.append(PROJECT_ID, project_id.to_string().parse()?);

        let resp = HTTP
            .put(format!("{}/system/user/status", self.addr))
            .headers(headers)
            .json(req)
            .send()
            .await?;
        let status = resp.status();
        if !status.is_success() {
            let error_body = resp.text().await?;
            return Ok((
                status,
                AppResponseResult::Ok(MessageResponse::new(error_body)),
            ));
        }
        Ok((resp.status(), resp.json().await?))
    }

    #[logfn(Info)]
    pub async fn delete_user(
        &self,
        token: &str,
        project_id: i32,
        req: &DeleteUserRequest,
    ) -> anyhow::Result<(StatusCode, AppResponseResult)> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );
        headers.append(PROJECT_ID, project_id.to_string().parse()?);
        let resp = HTTP
            .delete(format!("{}/system/user", self.addr))
            .headers(headers)
            .json(req)
            .send()
            .await?;
        Ok((resp.status(), resp.json().await?))
    }

    #[logfn(Info)]
    pub async fn get_user_list(
        &self,
        token: &str,
        project_id: i32,
    ) -> anyhow::Result<(StatusCode, AppResponseResult<ListUserResponse>)> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );
        headers.append(PROJECT_ID, project_id.to_string().parse()?);
        let resp = HTTP
            .get(format!("{}/system/user/list", self.addr))
            .headers(headers)
            .send()
            .await?;
        Ok((resp.status(), resp.json().await?))
    }

    #[logfn(Info)]
    pub async fn delete_role(
        &self,
        token: &str,
        project_id: i32,
        req: &DeleteRoleRequest,
    ) -> anyhow::Result<(StatusCode, AppResponseResult<MessageResponse>)> {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );
        headers.append(PROJECT_ID, project_id.to_string().parse()?);
        let resp = HTTP
            .delete(format!("{}/system/user/role", self.addr))
            .headers(headers)
            .json(req)
            .send()
            .await?;
        Ok((resp.status(), resp.json().await?))
    }

    #[logfn(Info)]
    pub async fn get_user_role(
        &self,
        token: &str,
        project_id: i32,
        role_id: i32,
    ) -> anyhow::Result<(StatusCode, AppResponseResult<UserRole>)> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );
        headers.append(PROJECT_ID, project_id.to_string().parse()?);
        let resp = HTTP
            .get(format!("{}/system/user/role/{}", self.addr, role_id))
            .headers(headers)
            .send()
            .await?;
        Ok((resp.status(), resp.json().await?))
    }

    #[logfn(Info)]
    pub async fn get_role_permission(
        &self,
        token: &str,
        project_id: i32,
        role_id: i32,
    ) -> anyhow::Result<(StatusCode, AppResponseResult<Vec<Permission>>)> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );
        headers.append(PROJECT_ID, project_id.to_string().parse()?);
        let resp = HTTP
            .get(format!(
                "{}/system/user/role/permission/{}",
                self.addr, role_id
            ))
            .headers(headers)
            .send()
            .await?;
        Ok((resp.status(), resp.json().await?))
    }

    #[logfn(Info)]
    pub async fn get_role_permission_list(
        &self,
        token: &str,
        project_id: i32,
    ) -> anyhow::Result<(StatusCode, AppResponseResult<Vec<UserRolePermission>>)> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );
        headers.append(PROJECT_ID, project_id.to_string().parse()?);
        let resp = HTTP
            .get(format!("{}/system/user/role/permission/list", self.addr))
            .headers(headers)
            .send()
            .await?;
        Ok((resp.status(), resp.json().await?))
    }

    #[logfn(Info)]
    pub async fn create_case_module(
        &self,
        token: &str,
        project_id: i32,
        req: &CreateModuleRequest,
    ) -> anyhow::Result<(StatusCode, AppResponseResult<CreateEntityResponse>)> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );
        headers.append(PROJECT_ID, project_id.to_string().parse()?);

        let resp = HTTP
            .post(format!("{}/management/case/module", self.addr))
            .headers(headers)
            .json(req)
            .send()
            .await?;

        Ok((resp.status(), resp.json().await?))
    }

    #[logfn(Info)]
    pub async fn create_functional_case(
        &self,
        token: &str,
        project_id: i32,
        req: &CreateFunctionalCaseRequest,
    ) -> anyhow::Result<(StatusCode, AppResponseResult<CreateEntityResponse>)> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );

        headers.append(PROJECT_ID, project_id.to_string().parse()?);
        let resp = HTTP
            .post(format!("{}/management/case/functional-case", self.addr))
            .headers(headers)
            .json(req)
            .send()
            .await?;

        Ok((resp.status(), resp.json().await?))
    }

    #[logfn(Info)]
    pub async fn get_functional_case(
        &self,
        token: &str,
        project_id: i32,
        case_id: i32,
    ) -> anyhow::Result<(StatusCode, AppResponseResult<FunctionalCaseResponse>)> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );
        headers.append(PROJECT_ID, project_id.to_string().parse()?);

        let resp = HTTP
            .get(format!(
                "{}/management/case/functional-case/{}",
                self.addr, case_id
            ))
            .headers(headers)
            .send()
            .await?;

        Ok((resp.status(), resp.json().await?))
    }

    #[logfn(Info)]
    pub async fn get_functional_case_list(
        &self,
        token: &str,
        project_id: i32,
        params: &Option<ListQueryParam>,
    ) -> anyhow::Result<(StatusCode, AppResponseResult<ListFunctionalCaseResponse>)> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );
        headers.append(PROJECT_ID, project_id.to_string().parse()?);
        let resp = HTTP
            .get(format!("{}/management/case/functional-case", self.addr))
            .headers(headers)
            .query(params)
            .send()
            .await?;

        Ok((resp.status(), resp.json().await?))
    }

    #[logfn(Info)]
    pub async fn update_functional_case(
        &self,
        token: &str,
        project_id: i32,
        req: &UpdateFunctionalCaseRequest,
    ) -> anyhow::Result<(StatusCode, AppResponseResult)> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );
        headers.append(PROJECT_ID, project_id.to_string().parse()?);

        let resp = HTTP
            .put(format!("{}/management/case/functional-case", self.addr))
            .headers(headers)
            .json(req)
            .send()
            .await?;

        Ok((resp.status(), resp.json().await?))
    }

    #[logfn(Info)]
    pub async fn get_case_module(
        &self,
        token: &str,
        project_id: i32,
        query_project_id: i32,
        query: &QueryModuleParam,
    ) -> anyhow::Result<(StatusCode, AppResponseResult<Vec<FileModuleResponse>>)> {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );
        headers.append(PROJECT_ID, project_id.to_string().parse()?);
        let resp = HTTP
            .get(format!(
                "{}/management/case/module/{}",
                self.addr, query_project_id
            ))
            .query(query)
            .headers(headers)
            .send()
            .await?;

        Ok((resp.status(), resp.json().await?))
    }

    #[logfn(Info)]
    pub async fn delete_case_module(
        &self,
        token: &str,
        project_id: i32,
        req: &DeleteModuleRequest,
    ) -> anyhow::Result<(StatusCode, AppResponseResult)> {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );

        headers.append(PROJECT_ID, project_id.to_string().parse()?);

        let resp = HTTP
            .delete(format!("{}/management/case/module", self.addr))
            .headers(headers)
            .json(req)
            .send()
            .await?;

        Ok((resp.status(), resp.json().await?))
    }

    pub async fn create_field(
        &self,
        token: &str,
        project_id: i32,
        req: &CreateFieldRequest,
    ) -> anyhow::Result<(StatusCode, AppResponseResult<CreateEntityResponse>)> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );
        headers.append(PROJECT_ID, project_id.to_string().parse()?);
        let resp = HTTP
            .post(format!("{}/management/case/field", self.addr))
            .headers(headers)
            .json(req)
            .send()
            .await?;

        Ok((resp.status(), resp.json().await?))
    }

    pub async fn update_field(
        &self,
        token: &str,
        project_id: i32,
        req: &UpdateFieldRequest,
    ) -> anyhow::Result<(StatusCode, AppResponseResult)> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );
        headers.append(PROJECT_ID, project_id.to_string().parse()?);
        let resp = HTTP
            .put(format!("{}/management/case/field", self.addr))
            .headers(headers)
            .json(req)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let error_body = resp.text().await?;
            return Ok((
                status,
                AppResponseResult::Ok(MessageResponse::new(error_body)),
            ));
        }
        Ok((resp.status(), resp.json().await?))
    }

    pub async fn delete_field(
        &self,
        token: &str,
        project_id: i32,
        req: &DeleteFieldRequest,
    ) -> anyhow::Result<(StatusCode, AppResponseResult)> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );
        headers.append(PROJECT_ID, project_id.to_string().parse()?);
        let resp = HTTP
            .delete(format!("{}/management/case/field", self.addr))
            .headers(headers)
            .json(req)
            .send()
            .await?;

        Ok((resp.status(), resp.json().await?))
    }

    pub async fn get_field_list(
        &self,
        token: &str,
        project_id: i32,
        query_project_id: i32,
        params: &QueryFieldParam,
    ) -> anyhow::Result<(StatusCode, AppResponseResult<Vec<Field>>)> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse()?,
        );
        headers.append(PROJECT_ID, project_id.to_string().parse()?);
        let resp = HTTP
            .get(format!(
                "{}/management/case/field/{}",
                self.addr, query_project_id
            ))
            .headers(headers)
            .query(params)
            .send()
            .await?;

        Ok((resp.status(), resp.json().await?))
    }
}
