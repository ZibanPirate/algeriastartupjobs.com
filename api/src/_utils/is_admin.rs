use axum::http::header::HeaderMap;

use crate::_entry::state::AppState;

pub fn is_admin(app_state: &AppState, headers: HeaderMap) -> Option<()> {
  let admin_auth_code_header = headers.get("x-admin-auth-code");
  match admin_auth_code_header {
    Some(header_value) => {
      let admin_code = &app_state.config_service.get_config().admin_auth_code;
      if header_value == admin_code {
        Some(())
      } else {
        None
      }
    }
    None => None,
  }
}
