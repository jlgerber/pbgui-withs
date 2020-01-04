
use qt_widgets::{QApplication, QShortcut};
use qt_core::{Key,QResource, Slot};
use listitem::{utility::qs, withlist::WithList};
use qt_gui::{QKeySequence};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    QApplication::init(|_app| unsafe {

        let _result = QResource::register_resource_q_string(&qs("/Users/jgerber/bin/withlist.rcc"));

        let  with_list = Rc::new(
            RefCell::new(
                WithList::new()
            )
        );
        let wl_c1 = with_list.clone();
        let wl_c2 = with_list.clone();
        with_list.borrow_mut().set_stylesheet("/Users/jgerber/bin/withlist.qss");

        let find_slot: Slot<'static> = Slot::new(move || {
            wl_c1.borrow_mut().set_find_mode();
        });

        let add_slot: Slot<'static> = Slot::new(move || {
            wl_c2.borrow_mut().set_add_mode();
        });

        //let key_seq = QKeySequence::from_2_int(Key::KeyControl.to_int(), Key::KeyF.to_int());
        //let key_seq = QKeySequence::from_int( Key::KeyF.to_int());
       let key_seq = QKeySequence::from_q_string(&qs("Ctrl+f"));
        let find_shortcut = QShortcut::new_2a(key_seq.as_ref(), with_list.borrow_mut().main());

        //let key_seq = QKeySequence::from_2_int(Key::KeyControl.to_int(), Key::KeyA.to_int());
        let key_seq = QKeySequence::from_q_string(&qs("Ctrl+a"));
        //let key_seq = QKeySequence::from_int( Key::KeyA.to_int());
        let add_shortcut = QShortcut::new_2a(key_seq.as_ref(), with_list.borrow_mut().main());

        with_list.borrow_mut().add_items(vec![
            "amtools",
            "animcomp",
            "animpublish",
            "animrender",
            "assetbrowser",
            "assetmanager",
            "atomic",
            "autorender",
            "dd",
            "ddg",
            "deferredpipeline",
            "gcc",
            "houdini",
            "houdinipipeline",
            "houdinisubmission",
            "jsconfig",
            "jstools",
            "jsutils",
            "layoutpipelne",
            "lightpipeline",
            "make",
            "mari",
            "maya",
            "modelpipeline",
            "modelpublish",
            "mudbox",
            "nuke",
            "nukesubmission",
            "organic",
            "packaboo",
            "packaboo_utils",
            "packrat",
            "pk",
            "pbutils",
            "prez",
            "qt",
            "qtpy",
            "race",
            "racetrack",
            "raceview",
            "redshift",
            "rigtools",
            "samson",
            "shotgun",
            "shotgunapi",
            "submission",
            "texturepublish",
            "texturepipeline",
            "vray",
            "vrayddbase",
            "vray_for_maya",
            "wam",
            "wambase",
            "xerces",
        ]);
        find_shortcut.activated().connect(&find_slot);
        add_shortcut.activated().connect(&add_slot);

        with_list.borrow_mut().item_list.borrow_mut().set_add_mode();

        with_list.borrow_mut().show();

        QApplication::exec()
    });
}
