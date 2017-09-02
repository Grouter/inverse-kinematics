package com.grouter.main;

import com.badlogic.gdx.ApplicationAdapter;
import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.InputProcessor;
import com.badlogic.gdx.graphics.GL20;
import com.badlogic.gdx.graphics.glutils.ShapeRenderer;

public class Main extends ApplicationAdapter implements InputProcessor {

    static final float SEGMENT_WIDTH = 5f;

    private Tentacle tentacle;
	private ShapeRenderer shapeRenderer;

	@Override
	public void create () {
        float width = Gdx.graphics.getWidth();
        float height = Gdx.graphics.getHeight();

		shapeRenderer = new ShapeRenderer();

		Gdx.input.setInputProcessor(this);

		tentacle = new Tentacle(width/2, height, 20, height/2);
	}

	@Override
	public void render () {
		Gdx.gl.glClearColor(0.2f, 0.2f, 0.2f, 1);
		Gdx.gl.glClear(GL20.GL_COLOR_BUFFER_BIT);

		tentacle.update();

		shapeRenderer.begin(ShapeRenderer.ShapeType.Filled);
		tentacle.render(shapeRenderer);
		shapeRenderer.end();
	}
	
	@Override
	public void dispose () {
		shapeRenderer.dispose();
	}

    @Override
    public boolean keyDown(int keycode) {
        return false;
    }

    @Override
    public boolean keyUp(int keycode) {
        return false;
    }

    @Override
    public boolean keyTyped(char character) {
        return false;
    }

    @Override
    public boolean touchDown(int screenX, int screenY, int pointer, int button) {
        return false;
    }

    @Override
    public boolean touchUp(int screenX, int screenY, int pointer, int button) {
        return false;
    }

    @Override
    public boolean touchDragged(int screenX, int screenY, int pointer) {
        return false;
    }

    @Override
    public boolean mouseMoved(int screenX, int screenY) {
        return false;
    }

    @Override
    public boolean scrolled(int amount) {
        return false;
    }
}
