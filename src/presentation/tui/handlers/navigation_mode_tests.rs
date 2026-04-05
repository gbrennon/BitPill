#[cfg(test)]
mod navigation_mode_tests {
    use crossterm::event::KeyCode;

    use crate::presentation::tui::{
        app::App,
        handlers::{
            create_medication_handler::CreateMedicationHandler,
            edit_medication_handler::EditMedicationHandler, event_handler::EventHandler,
            port::Handler,
        },
        input::Key,
        screen::Screen,
    };

    fn key(c: char) -> Key {
        crate::presentation::tui::input::from_code(KeyCode::Char(c))
    }

    fn app_vim() -> App {
        App::new_fake_with_mode("vi")
    }

    fn app_emacs() -> App {
        App::new_fake_with_mode("emacs")
    }

    fn med(
        id: &str,
        name: &str,
        amount: u32,
    ) -> crate::application::dtos::responses::MedicationDto {
        crate::application::dtos::responses::MedicationDto {
            id: id.into(),
            name: name.into(),
            amount_mg: amount,
            scheduled_time: vec![],
            dose_frequency: "OnceDaily".into(),
            taken_today: 0,
            scheduled_today: 0,
        }
    }

    fn dose_record(id: &str, med_id: &str) -> crate::application::dtos::responses::DoseRecordDto {
        use chrono::Utc;

        use crate::application::dtos::responses::DoseRecordDto;
        DoseRecordDto {
            id: id.to_string(),
            medication_id: med_id.to_string(),
            scheduled_at: Utc::now().naive_utc(),
            taken_at: None,
        }
    }

    mod home_screen {
        use super::*;

        #[test]
        fn vim_mode_j_moves_down() {
            let mut app = app_vim();
            app.medications = vec![med("med1", "Med 1", 100), med("med2", "Med 2", 200)];

            let initial = app.selected_index;
            let mut h = EventHandler::default();
            h.handle(&mut app, key('j'));

            assert_eq!(app.selected_index, initial + 1);
        }

        #[test]
        fn vim_mode_k_moves_up() {
            let mut app = app_vim();
            app.medications = vec![med("med1", "Med 1", 100), med("med2", "Med 2", 200)];
            app.selected_index = 1;

            let mut h = EventHandler::default();
            h.handle(&mut app, key('k'));

            assert_eq!(app.selected_index, 0);
        }

        #[test]
        fn vim_mode_h_moves_left() {
            let mut app = app_vim();
            app.medications = vec![med("med1", "Med 1", 100), med("med2", "Med 2", 200)];
            app.selected_index = 1;

            let mut h = EventHandler::default();
            h.handle(&mut app, key('h'));

            assert_eq!(app.selected_index, 0);
        }

        #[test]
        fn vim_mode_l_moves_right() {
            let mut app = app_vim();
            app.medications = vec![med("med1", "Med 1", 100), med("med2", "Med 2", 200)];

            let initial = app.selected_index;
            let mut h = EventHandler::default();
            h.handle(&mut app, key('l'));

            assert_eq!(app.selected_index, initial + 1);
        }

        #[test]
        fn emacs_mode_n_moves_down() {
            let mut app = app_emacs();
            app.medications = vec![med("med1", "Med 1", 100), med("med2", "Med 2", 200)];

            let initial = app.selected_index;
            let mut h = EventHandler::default();
            h.handle(&mut app, key('n'));

            assert_eq!(app.selected_index, initial + 1);
        }

        #[test]
        fn emacs_mode_p_moves_up() {
            let mut app = app_emacs();
            app.medications = vec![med("med1", "Med 1", 100), med("med2", "Med 2", 200)];
            app.selected_index = 1;

            let mut h = EventHandler::default();
            h.handle(&mut app, key('p'));

            assert_eq!(app.selected_index, 0);
        }

        #[test]
        fn vim_j_key_fails_in_emacs_mode() {
            let mut app = app_emacs();
            app.medications = vec![med("med1", "Med 1", 100), med("med2", "Med 2", 200)];

            let initial = app.selected_index;
            let mut h = EventHandler::default();
            h.handle(&mut app, key('j'));

            assert_eq!(app.selected_index, initial);
        }

        #[test]
        fn vim_k_key_fails_in_emacs_mode() {
            let mut app = app_emacs();
            app.medications = vec![med("med1", "Med 1", 100), med("med2", "Med 2", 200)];
            app.selected_index = 1;

            let initial = app.selected_index;
            let mut h = EventHandler::default();
            h.handle(&mut app, key('k'));

            assert_eq!(app.selected_index, initial);
        }

        #[test]
        fn emacs_n_key_fails_in_vim_mode() {
            let mut app = app_vim();
            app.medications = vec![med("med1", "Med 1", 100), med("med2", "Med 2", 200)];

            let initial = app.selected_index;
            let mut h = EventHandler::default();
            h.handle(&mut app, key('n'));

            assert_eq!(app.selected_index, initial);
        }

        #[test]
        fn emacs_p_key_fails_in_vim_mode() {
            let mut app = app_vim();
            app.medications = vec![med("med1", "Med 1", 100), med("med2", "Med 2", 200)];
            app.selected_index = 1;

            let initial = app.selected_index;
            let mut h = EventHandler::default();
            h.handle(&mut app, key('p'));

            assert_eq!(app.selected_index, initial);
        }
    }

    mod settings_screen {
        use super::*;

        #[test]
        fn vim_mode_j_moves_down() {
            let mut app = app_vim();
            app.current_screen = Screen::Settings {
                vim_enabled: true,
                selected_index: 0,
            };

            let mut h = EventHandler::default();
            h.handle(&mut app, key('j'));

            if let Screen::Settings { selected_index, .. } = &app.current_screen {
                assert_eq!(*selected_index, 1);
            } else {
                panic!("Expected Settings screen");
            }
        }

        #[test]
        fn vim_mode_k_moves_up() {
            let mut app = app_vim();
            app.current_screen = Screen::Settings {
                vim_enabled: true,
                selected_index: 1,
            };

            let mut h = EventHandler::default();
            h.handle(&mut app, key('k'));

            if let Screen::Settings { selected_index, .. } = &app.current_screen {
                assert_eq!(*selected_index, 0);
            } else {
                panic!("Expected Settings screen");
            }
        }

        #[test]
        fn emacs_mode_n_moves_down() {
            let mut app = app_emacs();
            app.current_screen = Screen::Settings {
                vim_enabled: false,
                selected_index: 0,
            };

            let mut h = EventHandler::default();
            h.handle(&mut app, key('n'));

            if let Screen::Settings { selected_index, .. } = &app.current_screen {
                assert_eq!(*selected_index, 1);
            } else {
                panic!("Expected Settings screen");
            }
        }

        #[test]
        fn emacs_mode_p_moves_up() {
            let mut app = app_emacs();
            app.current_screen = Screen::Settings {
                vim_enabled: false,
                selected_index: 1,
            };

            let mut h = EventHandler::default();
            h.handle(&mut app, key('p'));

            if let Screen::Settings { selected_index, .. } = &app.current_screen {
                assert_eq!(*selected_index, 0);
            } else {
                panic!("Expected Settings screen");
            }
        }

        #[test]
        fn vim_j_key_fails_in_emacs_mode() {
            let mut app = app_emacs();
            app.current_screen = Screen::Settings {
                vim_enabled: false,
                selected_index: 0,
            };

            let mut h = EventHandler::default();
            h.handle(&mut app, key('j'));

            if let Screen::Settings { selected_index, .. } = &app.current_screen {
                assert_eq!(*selected_index, 0);
            } else {
                panic!("Expected Settings screen");
            }
        }

        #[test]
        fn emacs_n_key_fails_in_vim_mode() {
            let mut app = app_vim();
            app.current_screen = Screen::Settings {
                vim_enabled: true,
                selected_index: 0,
            };

            let mut h = EventHandler::default();
            h.handle(&mut app, key('n'));

            if let Screen::Settings { selected_index, .. } = &app.current_screen {
                assert_eq!(*selected_index, 0);
            } else {
                panic!("Expected Settings screen");
            }
        }
    }

    mod mark_dose_screen {
        use super::*;

        #[test]
        fn vim_mode_j_moves_down() {
            let mut app = app_vim();
            app.current_screen = Screen::MarkDose {
                medication_id: "med1".to_string(),
                records: vec![dose_record("r1", "med1"), dose_record("r2", "med1")],
                selected_index: 0,
            };

            let mut h = EventHandler::default();
            h.handle(&mut app, key('j'));

            if let Screen::MarkDose { selected_index, .. } = &app.current_screen {
                assert_eq!(*selected_index, 1);
            } else {
                panic!("Expected MarkDose screen");
            }
        }

        #[test]
        fn vim_mode_k_moves_up() {
            let mut app = app_vim();
            app.current_screen = Screen::MarkDose {
                medication_id: "med1".to_string(),
                records: vec![dose_record("r1", "med1"), dose_record("r2", "med1")],
                selected_index: 1,
            };

            let mut h = EventHandler::default();
            h.handle(&mut app, key('k'));

            if let Screen::MarkDose { selected_index, .. } = &app.current_screen {
                assert_eq!(*selected_index, 0);
            } else {
                panic!("Expected MarkDose screen");
            }
        }

        #[test]
        fn emacs_mode_n_moves_down() {
            let mut app = app_emacs();
            app.current_screen = Screen::MarkDose {
                medication_id: "med1".to_string(),
                records: vec![dose_record("r1", "med1"), dose_record("r2", "med1")],
                selected_index: 0,
            };

            let mut h = EventHandler::default();
            h.handle(&mut app, key('n'));

            if let Screen::MarkDose { selected_index, .. } = &app.current_screen {
                assert_eq!(*selected_index, 1);
            } else {
                panic!("Expected MarkDose screen");
            }
        }

        #[test]
        fn emacs_mode_p_moves_up() {
            let mut app = app_emacs();
            app.current_screen = Screen::MarkDose {
                medication_id: "med1".to_string(),
                records: vec![dose_record("r1", "med1"), dose_record("r2", "med1")],
                selected_index: 1,
            };

            let mut h = EventHandler::default();
            h.handle(&mut app, key('p'));

            if let Screen::MarkDose { selected_index, .. } = &app.current_screen {
                assert_eq!(*selected_index, 0);
            } else {
                panic!("Expected MarkDose screen");
            }
        }

        #[test]
        fn vim_j_key_fails_in_emacs_mode() {
            let mut app = app_emacs();
            app.current_screen = Screen::MarkDose {
                medication_id: "med1".to_string(),
                records: vec![dose_record("r1", "med1"), dose_record("r2", "med1")],
                selected_index: 0,
            };

            let mut h = EventHandler::default();
            h.handle(&mut app, key('j'));

            if let Screen::MarkDose { selected_index, .. } = &app.current_screen {
                assert_eq!(*selected_index, 0);
            } else {
                panic!("Expected MarkDose screen");
            }
        }

        #[test]
        fn emacs_n_key_fails_in_vim_mode() {
            let mut app = app_vim();
            app.current_screen = Screen::MarkDose {
                medication_id: "med1".to_string(),
                records: vec![dose_record("r1", "med1"), dose_record("r2", "med1")],
                selected_index: 0,
            };

            let mut h = EventHandler::default();
            h.handle(&mut app, key('n'));

            if let Screen::MarkDose { selected_index, .. } = &app.current_screen {
                assert_eq!(*selected_index, 0);
            } else {
                panic!("Expected MarkDose screen");
            }
        }
    }

    mod create_medication_screen {
        use super::*;

        #[test]
        fn vim_mode_j_moves_down() {
            let mut app = app_vim();
            app.current_screen = Screen::CreateMedication {
                name: String::new(),
                amount_mg: String::new(),
                selected_frequency: 0,
                scheduled_time: vec![String::new()],
                scheduled_idx: 0,
                focused_field: 0,
                insert_mode: false,
            };

            let mut h = CreateMedicationHandler::default();
            h.handle(&mut app, key('j'));

            if let Screen::CreateMedication { focused_field, .. } = &app.current_screen {
                assert_eq!(*focused_field, 1);
            } else {
                panic!("Expected CreateMedication screen");
            }
        }

        #[test]
        fn vim_mode_k_moves_up() {
            let mut app = app_vim();
            app.current_screen = Screen::CreateMedication {
                name: String::new(),
                amount_mg: String::new(),
                selected_frequency: 0,
                scheduled_time: vec![String::new()],
                scheduled_idx: 0,
                focused_field: 1,
                insert_mode: false,
            };

            let mut h = CreateMedicationHandler::default();
            h.handle(&mut app, key('k'));

            if let Screen::CreateMedication { focused_field, .. } = &app.current_screen {
                assert_eq!(*focused_field, 0);
            } else {
                panic!("Expected CreateMedication screen");
            }
        }

        #[test]
        fn emacs_mode_n_moves_down() {
            let mut app = app_emacs();
            app.current_screen = Screen::CreateMedication {
                name: String::new(),
                amount_mg: String::new(),
                selected_frequency: 0,
                scheduled_time: vec![String::new()],
                scheduled_idx: 0,
                focused_field: 0,
                insert_mode: false,
            };

            let mut h = CreateMedicationHandler::default();
            h.handle(&mut app, key('n'));

            if let Screen::CreateMedication { focused_field, .. } = &app.current_screen {
                assert_eq!(*focused_field, 1);
            } else {
                panic!("Expected CreateMedication screen");
            }
        }

        #[test]
        fn emacs_mode_p_moves_up() {
            let mut app = app_emacs();
            app.current_screen = Screen::CreateMedication {
                name: String::new(),
                amount_mg: String::new(),
                selected_frequency: 0,
                scheduled_time: vec![String::new()],
                scheduled_idx: 0,
                focused_field: 1,
                insert_mode: false,
            };

            let mut h = CreateMedicationHandler::default();
            h.handle(&mut app, key('p'));

            if let Screen::CreateMedication { focused_field, .. } = &app.current_screen {
                assert_eq!(*focused_field, 0);
            } else {
                panic!("Expected CreateMedication screen");
            }
        }

        #[test]
        fn vim_j_key_fails_in_emacs_mode() {
            let mut app = app_emacs();
            app.current_screen = Screen::CreateMedication {
                name: String::new(),
                amount_mg: String::new(),
                selected_frequency: 0,
                scheduled_time: vec![String::new()],
                scheduled_idx: 0,
                focused_field: 0,
                insert_mode: false,
            };

            let mut h = CreateMedicationHandler::default();
            h.handle(&mut app, key('j'));

            if let Screen::CreateMedication { focused_field, .. } = &app.current_screen {
                assert_eq!(*focused_field, 0);
            } else {
                panic!("Expected CreateMedication screen");
            }
        }

        #[test]
        fn emacs_n_key_fails_in_vim_mode() {
            let mut app = app_vim();
            app.current_screen = Screen::CreateMedication {
                name: String::new(),
                amount_mg: String::new(),
                selected_frequency: 0,
                scheduled_time: vec![String::new()],
                scheduled_idx: 0,
                focused_field: 0,
                insert_mode: false,
            };

            let mut h = CreateMedicationHandler::default();
            h.handle(&mut app, key('n'));

            if let Screen::CreateMedication { focused_field, .. } = &app.current_screen {
                assert_eq!(*focused_field, 0);
            } else {
                panic!("Expected CreateMedication screen");
            }
        }
    }

    mod edit_medication_screen {
        use super::*;

        #[test]
        fn vim_mode_j_moves_down() {
            let mut app = app_vim();
            app.current_screen = Screen::EditMedication {
                id: "med1".to_string(),
                name: String::new(),
                amount_mg: String::new(),
                selected_frequency: 0,
                scheduled_time: vec![String::new()],
                scheduled_idx: 0,
                focused_field: 0,
                insert_mode: false,
            };

            let mut h = EditMedicationHandler::default();
            h.handle(&mut app, key('j'));

            if let Screen::EditMedication { focused_field, .. } = &app.current_screen {
                assert_eq!(*focused_field, 1);
            } else {
                panic!("Expected EditMedication screen");
            }
        }

        #[test]
        fn vim_mode_k_moves_up() {
            let mut app = app_vim();
            app.current_screen = Screen::EditMedication {
                id: "med1".to_string(),
                name: String::new(),
                amount_mg: String::new(),
                selected_frequency: 0,
                scheduled_time: vec![String::new()],
                scheduled_idx: 0,
                focused_field: 1,
                insert_mode: false,
            };

            let mut h = EditMedicationHandler::default();
            h.handle(&mut app, key('k'));

            if let Screen::EditMedication { focused_field, .. } = &app.current_screen {
                assert_eq!(*focused_field, 0);
            } else {
                panic!("Expected EditMedication screen");
            }
        }

        #[test]
        fn emacs_mode_n_moves_down() {
            let mut app = app_emacs();
            app.current_screen = Screen::EditMedication {
                id: "med1".to_string(),
                name: String::new(),
                amount_mg: String::new(),
                selected_frequency: 0,
                scheduled_time: vec![String::new()],
                scheduled_idx: 0,
                focused_field: 0,
                insert_mode: false,
            };

            let mut h = EditMedicationHandler::default();
            h.handle(&mut app, key('n'));

            if let Screen::EditMedication { focused_field, .. } = &app.current_screen {
                assert_eq!(*focused_field, 1);
            } else {
                panic!("Expected EditMedication screen");
            }
        }

        #[test]
        fn emacs_mode_p_moves_up() {
            let mut app = app_emacs();
            app.current_screen = Screen::EditMedication {
                id: "med1".to_string(),
                name: String::new(),
                amount_mg: String::new(),
                selected_frequency: 0,
                scheduled_time: vec![String::new()],
                scheduled_idx: 0,
                focused_field: 1,
                insert_mode: false,
            };

            let mut h = EditMedicationHandler::default();
            h.handle(&mut app, key('p'));

            if let Screen::EditMedication { focused_field, .. } = &app.current_screen {
                assert_eq!(*focused_field, 0);
            } else {
                panic!("Expected EditMedication screen");
            }
        }

        #[test]
        fn vim_j_key_fails_in_emacs_mode() {
            let mut app = app_emacs();
            app.current_screen = Screen::EditMedication {
                id: "med1".to_string(),
                name: String::new(),
                amount_mg: String::new(),
                selected_frequency: 0,
                scheduled_time: vec![String::new()],
                scheduled_idx: 0,
                focused_field: 0,
                insert_mode: false,
            };

            let mut h = EditMedicationHandler::default();
            h.handle(&mut app, key('j'));

            if let Screen::EditMedication { focused_field, .. } = &app.current_screen {
                assert_eq!(*focused_field, 0);
            } else {
                panic!("Expected EditMedication screen");
            }
        }

        #[test]
        fn emacs_n_key_fails_in_vim_mode() {
            let mut app = app_vim();
            app.current_screen = Screen::EditMedication {
                id: "med1".to_string(),
                name: String::new(),
                amount_mg: String::new(),
                selected_frequency: 0,
                scheduled_time: vec![String::new()],
                scheduled_idx: 0,
                focused_field: 0,
                insert_mode: false,
            };

            let mut h = EditMedicationHandler::default();
            h.handle(&mut app, key('n'));

            if let Screen::EditMedication { focused_field, .. } = &app.current_screen {
                assert_eq!(*focused_field, 0);
            } else {
                panic!("Expected EditMedication screen");
            }
        }
    }
}
