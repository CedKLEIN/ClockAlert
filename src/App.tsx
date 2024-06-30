import { useState, useEffect, useRef } from 'react';
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs';
import { LocalizationProvider } from '@mui/x-date-pickers/LocalizationProvider';
import { TimePicker } from '@mui/x-date-pickers/TimePicker';
import Button from '@mui/material/Button';
import AddAlertIcon from '@mui/icons-material/AddAlert';
import Clock from 'react-clock';
import { invoke } from '@tauri-apps/api';
import dayjs, { Dayjs } from 'dayjs';
import { listen } from '@tauri-apps/api/event';

import './App.css';
import 'react-clock/dist/Clock.css';
import 'react-time-picker/dist/TimePicker.css';

import AlarmSound from "./assets/audio.mp3";
  
interface Alarm {
  id: number;
  time: string;
}

function App() {
  const alarmTriggeredEvent: string = "alarm_triggered";
  const addAlarmCommand: string = "add_alarm";
  const removeAlarmCommand: string = "remove_alarm";
  const listAlarmsCommand: string = "list_alarms";

  const [currentTime, setCurrentTime] = useState(new Date());
  const [newAlarm, setNewAlarm] = useState<Dayjs | null>(dayjs());
  const [alarms, setAlarms] = useState<Alarm[]>([]);
  const [activeAlarmId, setActiveAlarmId] = useState<number | null>(null);
  const audio = useRef<HTMLAudioElement>(null);

  const convertToString = (date: dayjs.Dayjs | null): string => {
    return date ? date.format('HH:mm:ss') : '';
  };

  const handleAlarmTriggered = (payload: number ) => {
    setActiveAlarmId(payload);

    if (audio.current instanceof HTMLAudioElement) {
      audio.current.play();
    }
  };

  const handleStopAlarm = () => {
    setActiveAlarmId(null);

    if (audio.current instanceof HTMLAudioElement) {
      audio.current.pause();
      audio.current.currentTime = 0;
    }
  };
  
  useEffect(() => {
    const interval = setInterval(() => setCurrentTime(new Date()), 1000);
    const listener = async () => {
      try {
        await listen(alarmTriggeredEvent, (event) => {
          handleAlarmTriggered(event.payload as number);
        });
      } catch (error) {
          console.error('Error while listening for alarm_triggered event:', error);
      }
    };

    listener();
    fetchAlarms();
    return () => 
      {
        clearInterval(interval);
      };
  }, []);

  const handleAddAlarm = () => {
    try {
      invoke(addAlarmCommand, { time: convertToString(newAlarm) })
      .then(() => fetchAlarms())
      .catch((error) => console.error(error));        
    } catch (error) {
      console.error('Failed to add alarm:', error);
    }
  };

  const handleDeleteAlarm = (id: number) => {
    invoke(removeAlarmCommand, { id })
    .then(() => {
      if(id == activeAlarmId)
      {
        handleStopAlarm();
      }
      fetchAlarms();
    })
    .catch((error) => console.error('Failed to remove alarm:', error));
  };

  const fetchAlarms = () => {
    invoke(listAlarmsCommand)
    .then((response) => setAlarms(response as Alarm[]))
    .catch((error) => console.error('Failed to fetch alarms:', error));
  };

  return (
  <div className="container">
    <div className="left-panel">
      <h1>Alarms</h1>
      <div className="center-container">
        <LocalizationProvider dateAdapter={AdapterDayjs}>
          <TimePicker
            timeSteps={{ hours: 1, minutes: 1, seconds: 1 }}
            ampm={false}
            value={newAlarm}
            views={['hours', 'minutes', 'seconds']}
            format="HH:mm:ss"
            onChange={(newValue) => setNewAlarm(newValue)}
          />
        </LocalizationProvider>
        <Button variant="contained" 
              onClick={handleAddAlarm}
              size="large"><AddAlertIcon />
        </Button>
      </div>        

      <ul>
        {alarms.map((alarm, index) => (
          <li key={index}>
            {alarm.time}
            <div className="button-group">
              {activeAlarmId === alarm.id && (
                <Button
                  variant="contained"
                  color="secondary"
                  onClick={handleStopAlarm}
                  className="stop-button"
                >
                  Stop
                </Button>
              )}
              <button className="delete-button" onClick={() => handleDeleteAlarm(alarm.id)}>
                &#10060;
              </button>
            </div>
          </li>
        ))}
      </ul>
    </div>
    <div className="right-panel">
      <h1>Clock</h1>
      <div className="clock-container" >
        <Clock value={currentTime} size={300}/>
        <audio ref={audio} src={AlarmSound} />
      </div>
    </div>
  </div>
  );
}

export default App;
