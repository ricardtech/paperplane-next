use gtk::gio;
use gtk::glib;
use gtk::prelude::*;

use crate::application::Application;

pub(crate) fn init_tray(_app: &Application) {
    let connection = match gio::bus_get_sync(gio::BusType::Session, gio::Cancellable::NONE) {
        Ok(c) => c,
        Err(e) => {
            log::warn!("Could not connect to session D-Bus for tray: {e}");
            return;
        }
    };

    let xml = r#"
    <node>
        <interface name="org.kde.StatusNotifierItem">
            <method name="Activate">
                <arg name="x" type="i" direction="in"/>
                <arg name="y" type="i" direction="in"/>
            </method>
            <method name="SecondaryActivate">
                <arg name="x" type="i" direction="in"/>
                <arg name="y" type="i" direction="in"/>
            </method>
            <method name="ContextMenu">
                <arg name="x" type="i" direction="in"/>
                <arg name="y" type="i" direction="in"/>
            </method>
            <method name="Scroll">
                <arg name="delta" type="i" direction="in"/>
                <arg name="orientation" type="s" direction="in"/>
            </method>
            <property name="Category" type="s" access="read"/>
            <property name="Id" type="s" access="read"/>
            <property name="Title" type="s" access="read"/>
            <property name="Status" type="s" access="read"/>
            <property name="IconName" type="s" access="read"/>
            <property name="OverlayIconName" type="s" access="read"/>
            <property name="AttentionIconName" type="s" access="read"/>
            <property name="ItemIsMenu" type="b" access="read"/>
            <property name="IconThemePath" type="s" access="read"/>
            <property name="Menu" type="o" access="read"/>
        </interface>
    </node>
    "#;

    let node_info = match gio::DBusNodeInfo::for_xml(xml) {
        Ok(info) => info,
        Err(e) => {
            log::warn!("Failed to parse StatusNotifierItem XML: {e}");
            return;
        }
    };

    let iface = node_info.lookup_interface("org.kde.StatusNotifierItem").unwrap();

    let method_call = |_conn: gio::DBusConnection,
                       _sender: &str,
                       _object_path: &str,
                       _interface_name: &str,
                       method_name: &str,
                       _parameters: glib::Variant,
                       invocation: gio::DBusMethodInvocation| {
        log::info!("Tray method called: {method_name}");
        if method_name == "Activate" || method_name == "SecondaryActivate" {
            glib::idle_add_local_once(|| {
                toggle_window();
            });
        }
        invocation.return_value(None);
    };

    let get_property = |_conn: gio::DBusConnection,
                        _sender: &str,
                        _object_path: &str,
                        _interface_name: &str,
                        property_name: &str|
     -> glib::Variant {
        match property_name {
            "Category" => "Communications".to_variant(),
            "Id" => "app.drey.PaperPlane".to_variant(),
            "Title" => "PaperPlane Next".to_variant(),
            "Status" => "Active".to_variant(),
            "IconName" => "app.drey.PaperPlane".to_variant(),
            "OverlayIconName" => "".to_variant(),
            "AttentionIconName" => "".to_variant(),
            "ItemIsMenu" => false.to_variant(),
            "IconThemePath" => "".to_variant(),
            "Menu" => glib::Variant::parse(Some(glib::VariantTy::OBJECT_PATH), "\"/MenuBar\"").unwrap(),
            _ => ().to_variant(),
        }
    };

    let set_property = |_conn: gio::DBusConnection,
                        _sender: &str,
                        _object_path: &str,
                        _interface_name: &str,
                        _property_name: &str,
                        _value: glib::Variant|
     -> bool {
        false
    };

    if let Err(e) = connection.register_object(
        "/StatusNotifierItem",
        &iface,
        method_call,
        get_property,
        set_property,
    ) {
        log::warn!("Failed to register /StatusNotifierItem on D-Bus: {e}");
        return;
    }

    // Register with StatusNotifierWatcher
    let item_path = format!("{}/StatusNotifierItem", connection.unique_name().unwrap_or_default());
    let _ = connection.call(
        Some("org.kde.StatusNotifierWatcher"),
        "/StatusNotifierWatcher",
        "org.kde.StatusNotifierWatcher",
        "RegisterStatusNotifierItem",
        Some(&(item_path.to_variant())),
        None,
        gio::DBusCallFlags::NONE,
        -1,
        gio::Cancellable::NONE,
        |res| match res {
            Ok(_) => log::info!("StatusNotifierItem successfully registered with watcher!"),
            Err(e) => log::warn!("Failed to call RegisterStatusNotifierItem: {e}"),
        },
    );
}

pub(crate) fn toggle_window() {
    if let Some(app) = gio::Application::default() {
        if let Ok(app) = app.downcast::<Application>() {
            let win = app.main_window();
            let is_vis = win.is_visible();
            win.set_visible(!is_vis);
            if !is_vis {
                win.present();
            }
        }
    }
}
